use quote::quote;
use regex::{Captures, Regex};

use syn::ExprCall;
use syn::{parse::Parse, TraitItem};

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
                    let last_index = parsed_body.stmts.len() - 1;
                    // 遍历方法体中的每个语句
                    for (i, stmt) in parsed_body.stmts.into_iter().enumerate() {
                        let _is_last_stmt = i == last_index; // 检查是否是最后一个语句
                        let refined_stmt = match stmt {
                            // 带分号的表达式语句
                            syn::Stmt::Semi(ref expr, semi) => {
                                // 使用syn来分析表达式，判断是赋值表达式还是其他类型的表达式
                                match expr {
                                    syn::Expr::Assign(assign_expr) => syn::Stmt::Semi(
                                        process_assignment_expr(
                                            &re,
                                            &syn::Expr::Assign(assign_expr.clone()),
                                        ),
                                        semi,
                                    ),
                                    syn::Expr::AssignOp(assign_op_expr) => syn::Stmt::Semi(
                                        process_assignment_expr(
                                            &re,
                                            &syn::Expr::AssignOp(assign_op_expr.clone()),
                                        ),
                                        semi,
                                    ),

                                    // 如果表达式是宏调用, 将宏调用中的self.<field>替换为self._<field>()
                                    syn::Expr::Macro(expr_macro) => {
                                        let macro_tokens = &expr_macro.mac.tokens;
                                        let new_macro_str = replace_self_field(macro_tokens, true);
                                        // 将替换后的字符串转换回TokenStream
                                        let new_tokens =
                                            new_macro_str.parse().expect("Failed to parse tokens");
                                        // 构建新的宏调用表达式
                                        let new_macro = syn::Expr::Macro(syn::ExprMacro {
                                            attrs: expr_macro.attrs.clone(),
                                            mac: syn::Macro {
                                                path: expr_macro.mac.path.clone(),
                                                bang_token: expr_macro.mac.bang_token,
                                                delimiter: expr_macro.mac.delimiter.clone(),
                                                tokens: new_tokens,
                                            },
                                        });
                                        // 将新的宏调用表达式包装回Stmt::Semi
                                        syn::Stmt::Semi(new_macro, semi)
                                    }
                                    // for return statement
                                    syn::Expr::Return(ref expr_return) => {
                                        let replaced_expr_str = replace_self_field(expr_return, true);
                                        let replaced_expr = syn::parse_str(&replaced_expr_str)
                                            .expect("Failed to parse replaced expression in return statement");
                                        syn::Stmt::Semi(
                                            syn::Expr::Return(replaced_expr),
                                            semi,
                                        )
                                    }
                                    // 如果表达式是函数调用
                                    syn::Expr::Call(expr_call) => {
                                        let mut new_args = Vec::new();
                                        for arg in &expr_call.args {
                                            let new_arg_str = replace_self_field(arg, true);
                                            let new_arg: syn::Expr = syn::parse_str(&new_arg_str).expect("Failed to parse new arg");
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
                                        syn::Stmt::Semi(syn::Expr::Call(new_expr_call), semi)
                                    }
                                    // 其他类型的表达式(like trail expression)
                                    _ => {
                                        // TODO: block refine
                                        stmt.clone()
                                        // if is_last_stmt {
                                        //     // 如果是最后一个语句（尾表达式），则特殊处理
                                        //     let new_expr_str = replace_self_field(expr, false);
                                        //     let new_expr: syn::Expr = syn::parse_str(&new_expr_str)
                                        //         .expect("Failed to parse new expr");
                                        //     syn::Stmt::Expr(new_expr)
                                        // } else {
                                        //     // 如果不是尾表达式，保持原样
                                        //     stmt.clone()
                                        // }
                                    }
                                }
                            }
                            // 不带分号的表达式语句
                            syn::Stmt::Expr(ref expr) => {
                                let new_expr_str = replace_self_field(expr, true);
                                let new_expr: syn::Expr = syn::parse_str(&new_expr_str)
                                    .expect("Failed to parse new expr");
                                syn::Stmt::Expr(new_expr)
                            }
                            // 匹配局部变量绑定（let语句）
                            syn::Stmt::Local(local) => {
                                // 使用replace_self_field函数进行替换
                                let new_local_init = if let Some((_, init)) = local.init {
                                    Some((syn::token::Eq::default(), Box::new(syn::parse_str(&replace_self_field(&init, true)).expect("Failed to parse init expr"))))
                                } else {
                                    None
                                };
                                // 构建新的局部变量绑定
                                syn::Stmt::Local(syn::Local {
                                    init: new_local_init,
                                    ..local
                                })
                            },
                            // 非表达式的类型
                            _ => {
                                // stmt
                                //
                                let stmt_str = quote!(#stmt).to_string();
                                let new_stmt_str = re
                                    .replace_all(&stmt_str, |caps: &Captures| {
                                        // 检查是否需要考虑可变性
                                        let field_name = &caps[1];
                                        // 这里我们假设所有的字段访问都不需要考虑可变性，直接替换为self._<field_name>()
                                        // 实际情况可能需要根据上下文判断是否需要使用self._<field_name>_mut()
                                        format!("self._{}()", field_name)
                                    })
                                    .to_string();
                                let new_stmt: syn::Stmt = syn::parse_str(&new_stmt_str)
                                    .expect("Failed to parse new stmt");
                                new_stmt
                            }
                        };
                        new_stmts.push(refined_stmt);
                    }

                    // 将修改后的语句重新组装成方法体
                    let new_body = syn::Block {
                        brace_token: parsed_body.brace_token,
                        stmts: new_stmts,
                    };

                    // 将新的方法体设置回method.default
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
