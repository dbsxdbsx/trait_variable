use quote::quote;
use regex::Regex;

use syn::TraitItem;
use syn::{ExprCall, Local, LocalInit, Stmt};

use crate::trait_utils::{
    is_ref_mut, is_trait_method_mutable, parse_assignment_expr, replace_self_field, MyAssignOp,
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
                        let parsed_body: syn::Block = syn::parse2(quote! { #body })
                            .expect("Failed to parse trait method body");
                        // Iterate over each statement in the method body
                        let mut new_stmts = Vec::new();
                        let re = Regex::new(r"\bself\.([a-zA-Z_]\w*)").unwrap();
                        for stmt in parsed_body.stmts {
                            let refined_stmt = parse_stmt(&re, stmt, is_method_mut);
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
                _ => {
                    quote! { #item }
                }
            }
        })
        .collect::<Vec<_>>()
}

fn parse_stmt(re: &Regex, stmt: Stmt, is_method_mut: bool) -> Stmt {
    match stmt {
        Stmt::Macro(stmt_macro) => {
            let macro_tokens = &stmt_macro.mac.tokens;
            let new_macro_str = replace_self_field(macro_tokens, is_method_mut, false);
            Stmt::Macro(syn::StmtMacro {
                attrs: stmt_macro.attrs.clone(),
                mac: syn::Macro {
                    path: stmt_macro.mac.path.clone(),
                    bang_token: stmt_macro.mac.bang_token,
                    delimiter: stmt_macro.mac.delimiter.clone(),
                    tokens: new_macro_str.parse().expect("Failed to parse tokens"),
                },
                semi_token: stmt_macro.semi_token,
            })
        }
        Stmt::Expr(expr, semi) => Stmt::Expr(parse_expr(re, expr, is_method_mut, false), semi),
        // Local variable bindings (the `let` assignment pattern, like `let x = 42;`)
        Stmt::Local(assignment) => {
            // Convert the pattern to a string to check for `ref mut`
            let is_assignee_ref_mut = is_ref_mut(&quote!(#assignment.pat).to_string());
            // Process the initializer expression if it exists
            let new_local_init = if let Some(init) = assignment.init {
                Some(LocalInit {
                    eq_token: init.eq_token,
                    // The `expr` represents the right side of the assignment
                    expr: Box::new(parse_expr(
                        re,
                        *init.expr,
                        is_method_mut,
                        is_assignee_ref_mut,
                    )),
                    diverge: init.diverge,
                })
            } else {
                None
            };

            Stmt::Local(Local {
                init: new_local_init,
                ..assignment
            })
        }
        // Other cases remain the same
        _ => {
            unreachable!(
                "If you see this message, please check the logic of process_stmt function."
            )
        }
    }
}

fn parse_expr(
    re: &Regex,
    expr: syn::Expr,
    is_method_mut: bool,
    is_left_ref_mut: bool,
) -> syn::Expr {
    match expr {
        // TODO: delete?
        // syn::Expr::Macro(expr_macro) => {
        //     let macro_tokens = &expr_macro.mac.tokens;
        //     let new_macro_str = replace_self_field(macro_tokens, is_method_mut, is_left_ref_mut);
        //     syn::Expr::Macro(syn::ExprMacro {
        //         attrs: expr_macro.attrs.clone(),
        //         mac: syn::Macro {
        //             path: expr_macro.mac.path.clone(),
        //             bang_token: expr_macro.mac.bang_token,
        //             delimiter: expr_macro.mac.delimiter.clone(),
        //             tokens: new_macro_str.parse().expect("Failed to parse tokens"),
        //         },
        //     })
        // }
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
            // Check if the closure body is already a block expression
            let refined_body = match *closure.body {
                // If the closure body is a block expression, parse each statement in the block directly
                syn::Expr::Block(block) => {
                    let parsed_stmts: Vec<syn::Stmt> = block
                        .block
                        .stmts
                        .into_iter()
                        .map(|stmt| parse_stmt(re, stmt, is_method_mut))
                        .collect();
                    syn::Expr::Block(syn::ExprBlock {
                        block: syn::Block {
                            stmts: parsed_stmts,
                            ..block.block
                        },
                        ..block
                    })
                }
                // If the closure body is not a block expression, wrap it in a block and then parse it
                _ => {
                    let stmt = syn::Stmt::Expr(*closure.body, None);
                    match parse_stmt(re, stmt, is_method_mut) {
                        syn::Stmt::Expr(expr, _) => syn::Expr::Block(syn::ExprBlock {
                            block: syn::Block {
                                stmts: vec![syn::Stmt::Expr(expr, None)],
                                brace_token: Default::default(),
                            },
                            attrs: Vec::new(),
                            label: None, // no need label for closure of expression style
                        }),
                        _ => panic!("Unexpected stmt type after parsing closure body"),
                    }
                }
            };
            // Rebuild the closure with the refined body
            syn::Expr::Closure(syn::ExprClosure {
                body: Box::new(refined_body),
                ..closure
            })
        }
        _ => match MyAssignOp::from(expr.clone()) {
            MyAssignOp::AssignOp(assign_expr) => {
                parse_assignment_expr(assign_expr, is_method_mut)
                // todo!("Expression: {}", r.to_token_stream().to_string());
            }
            _ => {
                let new_expr_str = replace_self_field(&expr, is_method_mut, is_left_ref_mut);
                syn::parse_str(&new_expr_str).expect("Failed to parse new expr")
            }
        },
    }
}
