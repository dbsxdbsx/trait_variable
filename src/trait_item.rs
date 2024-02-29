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
        // TODO: expression with semicolon
        /*   Stmt::Expr(expr) | */
        Stmt::Semi(expr, semi) => syn::Stmt::Semi(process_expr(re, expr), semi),

        // TODO:expression without semicolon
        syn::Stmt::Expr(ref expr) => {
            let new_expr_str = replace_self_field(expr, true);
            let new_expr: syn::Expr =
                syn::parse_str(&new_expr_str).expect("Failed to parse new expr");
            syn::Stmt::Expr(new_expr)
        }
        // local variable bindings (let statements)
        syn::Stmt::Local(local) => {
            let new_local_init = if let Some((_, init)) = local.init {
                Some((
                    syn::token::Eq::default(),
                    Box::new(
                        syn::parse_str(&replace_self_field(&init, true))
                            .expect("Failed to parse init expr"),
                    ),
                ))
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
    // 使用syn来分析表达式，判断是赋值表达式还是其他类型的表达式
    match expr {
        syn::Expr::Assign(assign_expr) => {
            process_assignment_expr(&re, &syn::Expr::Assign(assign_expr.clone()))
        }
        syn::Expr::AssignOp(assign_op_expr) => {
            process_assignment_expr(&re, &syn::Expr::AssignOp(assign_op_expr.clone()))
        }
        syn::Expr::Macro(expr_macro) => {
            let macro_tokens = &expr_macro.mac.tokens;
            let new_macro_str = replace_self_field(macro_tokens, true);
            // 将替换后的字符串转换回TokenStream
            let new_tokens = new_macro_str.parse().expect("Failed to parse tokens");
            // 构建新的宏调用表达式
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
            // 构建新的函数调用表达式
            let new_expr_call = ExprCall {
                attrs: expr_call.attrs.clone(),
                func: expr_call.func.clone(),
                paren_token: expr_call.paren_token,
                args: new_args.into_iter().collect(),
            };
            // 将新的函数调用表达式包装回Stmt::Semi或Stmt::Expr
            syn::Expr::Call(new_expr_call)
        }
        // for lambda, closure
        syn::Expr::Closure(closure) => {
            // 处理闭包表达式
            todo!()
            // if let syn::Expr::Block(block) = *closure.body {
            //     // 确保闭包体是一个块表达式
            //     let processed_stmts: Vec<Stmt> =
            //         block.block.stmts.into_iter().map(process_stmt).collect();
            //     let new_block = syn::Block {
            //         stmts: processed_stmts,
            //         ..block.block
            //     };
            //     syn::Expr::Closure(syn::ExprClosure {
            //         body: Box::new(syn::Expr::Block(syn::ExprBlock {
            //             block: new_block,
            //             ..block
            //         })),
            //         ..closure
            //     })
            // } else {
            //     // 如果闭包体不是块表达式，直接返回原始闭包表达式
            //     syn::Expr::Closure(closure)
            // }
        }
        _ => expr,
    }
}
