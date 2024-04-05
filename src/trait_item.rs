use quote::quote;
use regex::{Captures, Regex};

use syn::{parse::Parse, TraitItem};
use syn::{parse_str, ExprCall, Local, LocalInit, Stmt};

use crate::utils::{
    is_ref_mut, is_trait_method_mutable, process_assignment_expr, replace_self_field,
};

pub fn refine_trait_items(trait_items: Vec<TraitItem>) -> Vec<proc_macro2::TokenStream> {
    trait_items
        .into_iter()
        .map(|item| {
            match item {
                TraitItem::Fn(mut trait_method) => {
                    // Determine if the method is mutable
                    let is_method_mut = is_trait_method_mutable(&trait_method);

                    // if the method has body, convert the trait variable fields into corresponding get-method
                    if let Some(body) = &mut trait_method.default {
                        // treat the body as `syn::Block`
                        let parsed_body: syn::Block =
                            syn::parse2(quote! { #body }).expect("Failed to parse method body");
                        let mut new_stmts = Vec::new();
                        let re = Regex::new(r"\bself\.([a-zA-Z_]\w*)").unwrap();
                        // Iterate over each statement in the method body
                        for stmt in parsed_body.stmts {
                            let refined_stmt = process_stmt(&re, stmt, is_method_mut);
                            new_stmts.push(refined_stmt);
                        }
                        // rebuild function body
                        let new_body = syn::Block {
                            brace_token: parsed_body.brace_token,
                            stmts: new_stmts,
                        };
                        trait_method.default = Some(
                            syn::parse(quote!(#new_body).into())
                                .expect("Failed to parse modified method body"),
                        );
                    }
                    // return the (refined, if it has body) method
                    quote! { #trait_method }
                }
                TraitItem::Type(trait_type) => {
                    quote! { #trait_type }
                }
                TraitItem::Const(trait_const) => {
                    quote! { #trait_const }
                }
                _ => {
                    quote! { #item }
                }
            }
        })
        .collect::<Vec<_>>()
}

fn process_stmt(re: &Regex, stmt: Stmt, is_method_mut: bool) -> Stmt {
    match stmt {
        // Expression with an optional trailing semicolon
        Stmt::Expr(expr, semi) => Stmt::Expr(process_expr(re, expr, is_method_mut, false), semi),
        // Local variable bindings (`let` assignments)
        Stmt::Local(local) => {
            // Convert the pattern to a string to check for `ref mut`
            let pat_str = quote!(#local.pat).to_string();
            let is_left_ref_mut = is_ref_mut(&pat_str);
            // Process the initializer expression if it exists
            let new_local_init = if let Some(init) = local.init {
                Some(LocalInit {
                    eq_token: init.eq_token,
                    expr: Box::new(process_expr(re, *init.expr, is_method_mut, is_left_ref_mut)),
                    diverge: init.diverge,
                })
            } else {
                None
            };

            Stmt::Local(Local {
                init: new_local_init,
                ..local
            })
        }
        // Other cases remain the same
        _ => {
            // TODO: check this block logic
            let stmt_str = quote!(#stmt).to_string();
            let new_stmt_str = re
                .replace_all(&stmt_str, |caps: &Captures| {
                    let field_name = &caps[1];
                    format!("self._{}()", field_name)
                })
                .to_string();
            let new_stmt: Stmt = parse_str(&new_stmt_str).expect("Failed to parse new stmt");
            new_stmt
        }
    }
}

fn process_expr(
    re: &Regex,
    expr: syn::Expr,
    is_method_mut: bool,
    is_left_ref_mut: bool,
) -> syn::Expr {
    match expr {
        syn::Expr::Assign(assign_expr) => {
            process_assignment_expr(re, &syn::Expr::Assign(assign_expr.clone()), is_method_mut)
        }
        syn::Expr::Binary(binary_expr) => {
            process_assignment_expr(re, &syn::Expr::Binary(binary_expr.clone()), is_method_mut)
        }
        syn::Expr::Macro(expr_macro) => {
            let macro_tokens = &expr_macro.mac.tokens;
            let new_macro_str = replace_self_field(macro_tokens, is_method_mut, is_left_ref_mut);
            syn::Expr::Macro(syn::ExprMacro {
                attrs: expr_macro.attrs.clone(),
                mac: syn::Macro {
                    path: expr_macro.mac.path.clone(),
                    bang_token: expr_macro.mac.bang_token,
                    delimiter: expr_macro.mac.delimiter.clone(),
                    tokens: new_macro_str.parse().expect("Failed to parse tokens"),
                },
            })
        }
        // for explicit return statement
        syn::Expr::Return(ref expr_return) => {
            let replaced_expr_str = replace_self_field(expr_return, is_method_mut, is_left_ref_mut);
            let replaced_expr = syn::parse_str(&replaced_expr_str)
                .expect("Failed to parse replaced expression in return statement");
            syn::Expr::Return(replaced_expr)
        }
        // for function call
        syn::Expr::Call(expr_call) => {
            let mut new_args = Vec::new();
            for arg in &expr_call.args {
                let new_arg_str = replace_self_field(arg, is_method_mut, is_left_ref_mut);
                let new_arg: syn::Expr =
                    syn::parse_str(&new_arg_str).expect("Failed to parse new arg");
                new_args.push(new_arg);
            }
            syn::Expr::Call(ExprCall {
                attrs: expr_call.attrs.clone(),
                func: expr_call.func.clone(),
                paren_token: expr_call.paren_token,
                args: new_args.into_iter().collect(),
            })
        }
        // for lambda, closure
        syn::Expr::Closure(closure) => {
            // 检查闭包体是否已经是一个块表达式
            let processed_body = match *closure.body {
                // 如果闭包体是一个块表达式，直接处理块中的每个语句
                syn::Expr::Block(block) => {
                    let processed_stmts: Vec<syn::Stmt> = block
                        .block
                        .stmts
                        .into_iter()
                        .map(|stmt| process_stmt(re, stmt, is_method_mut))
                        .collect();
                    syn::Expr::Block(syn::ExprBlock {
                        block: syn::Block {
                            stmts: processed_stmts,
                            ..block.block
                        },
                        ..block
                    })
                }
                // 如果闭包体不是一个块表达式，将其包装在一个块中然后处理
                _ => {
                    let stmt = syn::Stmt::Expr(*closure.body, None);
                    let processed_stmt = process_stmt(re, stmt, is_method_mut);
                    match processed_stmt {
                        syn::Stmt::Expr(expr, _) => syn::Expr::Block(syn::ExprBlock {
                            block: syn::Block {
                                stmts: vec![syn::Stmt::Expr(expr, None)],
                                brace_token: Default::default(),
                            },
                            attrs: Vec::new(),
                            label: None, // no need label for closure of expression style
                        }),
                        _ => panic!("Unexpected stmt type after processing closure body"),
                    }
                }
            };
            syn::Expr::Closure(syn::ExprClosure {
                body: Box::new(processed_body),
                ..closure
            })
        }
        _ => {
            let new_expr_str = replace_self_field(&expr, is_method_mut, is_left_ref_mut);
            syn::parse_str(&new_expr_str).expect("Failed to parse new expr")
        }
    }
}
