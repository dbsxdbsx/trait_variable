use quote::quote;
use regex::{Captures, Regex};

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
                        let is_last_stmt = i == last_index; // 检查是否是最后一个语句
                        let refined_stmt = match stmt {
                            // 对于表达式语句和带分号的表达式语句
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
                                    // 如果表达式是函数调用
                                    syn::Expr::Call(expr_call) => {
                                        // TODO: issue,
                                        // way1: use syn
                                        // 遍历函数调用的参数
                                        let mut new_args = Vec::new();
                                        for arg in &expr_call.args {
                                            if let syn::Expr::Reference(expr_ref) = arg {
                                                // 检查是否是 `&mut self.<field>` 形式
                                                if expr_ref.mutability.is_some() {
                                                    // 如果有 `mut` 关键字
                                                    if let syn::Expr::Path(expr_path) =
                                                        &*expr_ref.expr
                                                    {
                                                        if expr_path.path.is_ident("self") {
                                                            // 替换为 `self._<field>_mut()`
                                                            let field_name = expr_path
                                                                .path
                                                                .segments
                                                                .last()
                                                                .unwrap()
                                                                .ident
                                                                .to_string();
                                                            let new_arg =
                                                                syn::parse_str::<syn::Expr>(
                                                                    &format!(
                                                                        "self._{}_mut()",
                                                                        field_name
                                                                    ),
                                                                )
                                                                .unwrap();
                                                            new_args.push(new_arg);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            // 对于其他参数，使用原始参数
                                            new_args.push(arg.clone());
                                        }
                                        // way2: use regex, still wrong with compile error
                                        // // 定义正则表达式
                                        // let re_immutable =
                                        //     Regex::new(r"& self\.([a-zA-Z_]\w*)").unwrap();
                                        // let re_mutable =
                                        //     Regex::new(r"&mut self\.([a-zA-Z_]\w*)").unwrap();
                                        // // 遍历函数调用的参数
                                        // let mut new_args = Vec::new();
                                        // for arg in &expr_call.args {
                                        //     let arg_str = quote!(#arg).to_string();
                                        //     // 首先尝试匹配 &mut self.<field>
                                        //     let new_arg_str = if let Some(caps) =
                                        //         re_mutable.captures(&arg_str)
                                        //     {
                                        //         arg_str.replace(
                                        //             &caps[0],
                                        //             &format!("self._{}_mut()", &caps[1]),
                                        //         )
                                        //     } else if let Some(caps) = re_immutable.captures(&arg_str) {
                                        //         // 如果没有匹配到 &mut self.<field>，则尝试匹配 &self.<field>
                                        //         arg_str
                                        //             .replace(&caps[0], &format!("self._{}()", &caps[1]))
                                        //     } else {
                                        //         // 如果都没有匹配到，保持原样
                                        //         arg_str
                                        //     };
                                        //     // 将替换后的字符串转换回syn::Expr
                                        //     let new_arg: syn::Expr = syn::parse_str(&new_arg_str)
                                        //         .expect("Failed to parse new arg");
                                        //     new_args.push(new_arg);
                                        // }

                                        // 构建新的函数调用表达式
                                        let new_expr_call = syn::ExprCall {
                                            attrs: expr_call.attrs.clone(),
                                            func: expr_call.func.clone(),
                                            paren_token: expr_call.paren_token,
                                            args: syn::punctuated::Punctuated::from_iter(new_args),
                                        };
                                        // 将新的函数调用表达式包装回Stmt::Semi或Stmt::Expr
                                        syn::Stmt::Semi(syn::Expr::Call(new_expr_call), semi)
                                    }
                                    // 其他类型的表达式(like trail expression)
                                    _ => {
                                        // TODO: block refine
                                        if is_last_stmt {
                                            // 如果是最后一个语句（尾表达式），则特殊处理
                                            let new_expr_str = replace_self_field(expr, false);
                                            let new_expr: syn::Expr = syn::parse_str(&new_expr_str)
                                                .expect("Failed to parse new expr");
                                            syn::Stmt::Expr(new_expr)
                                        } else {
                                            // 如果不是尾表达式，保持原样
                                            stmt.clone()
                                        }
                                    }
                                }
                            }
                            syn::Stmt::Expr(ref expr) => {
                                let expr_str = quote!(#expr).to_string();
                                let new_expr_str = re
                                    .replace_all(&expr_str, |caps: &Captures| {
                                        format!("self._{}()", &caps[1])
                                    })
                                    .to_string();
                                let new_expr: syn::Expr = syn::parse_str(&new_expr_str)
                                    .expect("Failed to parse new expr");
                                syn::Stmt::Expr(new_expr)
                            }
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