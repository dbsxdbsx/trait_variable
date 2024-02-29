use quote::quote;
use regex::{Captures, Regex};

use syn::{parse::Parse, TraitItem};
use syn::{ExprCall, Stmt};

use crate::utils::{process_assignment_expr, replace_self_field};

pub fn refine_trait_items(trait_items: Vec<TraitItem>) -> Vec<proc_macro2::TokenStream> {
    trait_items
        .into_iter()
        .map(|item| {
            if let TraitItem::Method(mut method) = item {
                // if the method has body, convert the trait variable fields into corresponding get-method
                if let Some(body) = &mut method.default {
                    // 解析方法体为syn::Block
                    let parsed_body: syn::Block =
                        syn::parse2(quote! { #body }).expect("Failed to parse method body");
                    let mut new_stmts = Vec::new();
                    let re = Regex::new(r"\bself\.([a-zA-Z_]\w*)").unwrap();
                    // Iterate over each statement in the method body
                    for stmt in parsed_body.stmts {
                        let refined_stmt = process_stmt(&re, stmt);
                        new_stmts.push(refined_stmt);
                    }
                    // rebuild function body
                    let new_body = syn::Block {
                        brace_token: parsed_body.brace_token,
                        stmts: new_stmts,
                    };
                    method.default = Some(
                        syn::parse(quote!(#new_body).into())
                            .expect("Failed to parse modified method body"),
                    );
                }
                // return the (refined, if it has body) method
                quote! { #method }
            } else {
                // if it is not a method, just return it
                quote! { #item }
            }
        })
        .collect::<Vec<_>>()
}

fn process_stmt(re: &Regex, stmt: syn::Stmt) -> syn::Stmt {
    match stmt {
        Stmt::Semi(expr, semi) => syn::Stmt::Semi(process_expr(re, expr), semi),
        syn::Stmt::Expr(expr) => syn::Stmt::Expr(process_expr(re, expr)),
        // local variable bindings (let statements)
        syn::Stmt::Local(local) => {
            let new_local_init = if let Some((eq, init)) = local.init {
                // 使用 process_expr 函数处理初始化表达式
                let processed_init = process_expr(re, *init);
                Some((eq, Box::new(processed_init)))
            } else {
                None
            };
            syn::Stmt::Local(syn::Local {
                init: new_local_init,
                ..local
            })
        }
        _ => {
            // TODO: check this block logic
            let stmt_str = quote!(#stmt).to_string();
            let new_stmt_str = re
                .replace_all(&stmt_str, |caps: &Captures| {
                    let field_name = &caps[1];
                    format!("self._{}()", field_name)
                })
                .to_string();
            let new_stmt: syn::Stmt =
                syn::parse_str(&new_stmt_str).expect("Failed to parse new stmt");
            new_stmt
        }
    }
}

fn process_expr(re: &Regex, expr: syn::Expr) -> syn::Expr {
    match expr {
        syn::Expr::Assign(assign_expr) => {
            process_assignment_expr(re, &syn::Expr::Assign(assign_expr.clone()))
        }
        syn::Expr::AssignOp(assign_op_expr) => {
            process_assignment_expr(re, &syn::Expr::AssignOp(assign_op_expr.clone()))
        }
        syn::Expr::Macro(expr_macro) => {
            let macro_tokens = &expr_macro.mac.tokens;
            let new_macro_str = replace_self_field(macro_tokens, true);
            let new_tokens = new_macro_str.parse().expect("Failed to parse tokens");
            syn::Expr::Macro(syn::ExprMacro {
                attrs: expr_macro.attrs.clone(),
                mac: syn::Macro {
                    path: expr_macro.mac.path.clone(),
                    bang_token: expr_macro.mac.bang_token,
                    delimiter: expr_macro.mac.delimiter.clone(),
                    tokens: new_tokens,
                },
            })
        }
        // for explicit return statement
        syn::Expr::Return(ref expr_return) => {
            let replaced_expr_str = replace_self_field(expr_return, true);
            let replaced_expr = syn::parse_str(&replaced_expr_str)
                .expect("Failed to parse replaced expression in return statement");
            syn::Expr::Return(replaced_expr)
        }
        // for function call
        syn::Expr::Call(expr_call) => {
            let mut new_args = Vec::new();
            for arg in &expr_call.args {
                let new_arg_str = replace_self_field(arg, true);
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
                        .map(|stmt| process_stmt(re, stmt))
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
                    let stmt = syn::Stmt::Expr(*closure.body);
                    let processed_stmt = process_stmt(re, stmt);
                    match processed_stmt {
                        syn::Stmt::Expr(expr) => syn::Expr::Block(syn::ExprBlock {
                            block: syn::Block {
                                stmts: vec![syn::Stmt::Expr(expr)],
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
            let new_expr_str = replace_self_field(&expr, true);
            syn::parse_str(&new_expr_str).expect("Failed to parse new expr")
        }
    }
}
