extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use regex::{Captures, Regex};
use syn::{braced, parse_quote, token, Expr, ExprAssign, ExprPath, Member, Visibility};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, TraitItem, Type,
};

struct TraitVarField {
    var_vis: Visibility,
    var_name: Ident,
    _colon_token: Token![:],
    ty: Type,
}
impl Parse for TraitVarField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TraitVarField {
            var_vis: input.parse()?,
            var_name: input.parse()?,
            _colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

struct TraitInput {
    trait_vis: Visibility,
    _trait_token: Token![trait],
    trait_name: Ident,
    _brace_token: token::Brace,
    trait_variables: Punctuated<TraitVarField, Token![;]>,
    trait_items: Vec<TraitItem>,
}

impl Parse for TraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(TraitInput {
            trait_vis: input.parse()?,
            _trait_token: input.parse()?,
            trait_name: input.parse()?,
            _brace_token: braced!(content in input),
            // Parse all variable declarations until a method or end of input is encountered
            trait_variables: {
                let mut vars = Punctuated::new();
                while !content.peek(Token![fn]) && !content.peek(Token![;]) && !content.is_empty() {
                    vars.push_value(content.parse()?);
                    // Ensure that a semicolon follows the variable declaration
                    if !content.peek(Token![;]) {
                        return Err(content.error("expected `;` after variable declaration"));
                    }
                    vars.push_punct(content.parse()?);
                }
                vars
            },
            // Parse all method declarations
            trait_items: {
                let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }
                items
            },
        })
    }
}

/// functional macro: used to generate code for a trait with variable fields
#[proc_macro]
pub fn trait_variable(input: TokenStream) -> TokenStream {
    let TraitInput {
        trait_vis,
        trait_name,
        trait_variables,
        trait_items,
        ..
    } = parse_macro_input!(input as TraitInput);
    // 1.1 get parent trait name
    let parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    // 1.2 get trait declarative macro name
    let trait_decl_macro_name =
        Ident::new(&format!("{}_for_struct", trait_name), trait_name.span());

    // 2.1 generate parent trait methods declaration
    let parent_trait_methods =
        trait_variables
            .iter()
            .map(|TraitVarField { var_name, ty, .. }| {
                let method_name = Ident::new(&format!("_{}", var_name), var_name.span());
                let method_name_mut = Ident::new(&format!("_{}_mut", var_name), var_name.span());
                quote! {
                    fn #method_name(&self) -> &#ty;
                    fn #method_name_mut(&mut self) -> &mut #ty;
                }
            });
    // 2.2 generate trait variable fields definition for structs later
    let struct_trait_fields_defs = trait_variables.iter().map(
        |TraitVarField {
             var_vis,
             var_name,
             ty,
             ..
         }| {
            quote! {
                #var_vis #var_name: #ty,
            }
        },
    );
    // 2.3 generate parent trait methods implementation for struct
    let parent_trait_methods_impls =
        trait_variables
            .iter()
            .map(|TraitVarField { var_name, ty, .. }| {
                let method_name = Ident::new(&format!("_{}", var_name), var_name.span());
                let method_name_mut = Ident::new(&format!("_{}_mut", var_name), var_name.span());
                quote! {
                    fn #method_name(&self) -> &#ty{
                        &self.#var_name
                    }
                    fn #method_name_mut(&mut self) -> &mut #ty{
                        &mut self.#var_name
                    }
                }
            });

    // 3. refine the body of methods from the original trait
    let original_trait_items = trait_items.into_iter().map(|item| {
        if let TraitItem::Method(mut method) = item {
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
                                // 赋值表达式 // TODO: 只能等号？
                                syn::Expr::Assign(assign_expr) => {
                                    // 对赋值表达式的左侧进行处理
                                    let left = &assign_expr.left;
                                    let left_str = quote!(#left).to_string();
                                    let new_left_str = re
                                        .replace_all(&left_str, |caps: &Captures| {
                                            format!("(*self._{}_mut())", &caps[1])
                                        })
                                        .to_string();
                                    // 对赋值表达式的右侧进行处理
                                    let right = &assign_expr.right;
                                    let right_str = quote!(#right).to_string();
                                    let new_right_str = re
                                        .replace_all(&right_str, |caps: &Captures| {
                                            format!("self._{}()", &caps[1]) // TODO: add (*<original>)?
                                        })
                                        .to_string();
                                    // 重新构建赋值表达式
                                    let new_expr_str =
                                        format!("{} = {}", new_left_str, new_right_str);
                                    let new_expr: syn::Expr = syn::parse_str(&new_expr_str)
                                        .expect("Failed to parse new expr");
                                    // 根据原始语句的类型添加到新语句列表
                                    syn::Stmt::Semi(new_expr, semi)
                                }
                                // 复合赋值表达式，如 +=, -= 等
                                syn::Expr::AssignOp(assign_op_expr) => {
                                    let left = &assign_op_expr.left;
                                    let left_str = quote!(#left).to_string();
                                    let new_left_str = re
                                        .replace_all(&left_str, |caps: &Captures| {
                                            format!("(*self._{}_mut())", &caps[1])
                                        })
                                        .to_string();
                                    // 对赋值表达式的右侧进行处理
                                    let right = &assign_op_expr.right;
                                    let right_str = quote!(#right).to_string();
                                    let new_right_str = re
                                        .replace_all(&right_str, |caps: &Captures| {
                                            format!("self._{}()", &caps[1]) // TODO: add (*<original>)?
                                        })
                                        .to_string(); // 重新构建赋值表达式
                                    let new_expr_str = format!(
                                        "{} {} {}",
                                        new_left_str,
                                        assign_op_expr.op.to_token_stream(),
                                        new_right_str
                                    );
                                    let new_expr: syn::Expr = syn::parse_str(&new_expr_str)
                                        .expect("Failed to parse new expr");
                                    // 根据原始语句的类型添加到新语句列表
                                    syn::Stmt::Semi(new_expr, semi)
                                }
                                // 如果表达式是宏调用, 将宏调用中的self.<field>替换为self._<field>()
                                syn::Expr::Macro(expr_macro) => {
                                    let macro_tokens = &expr_macro.mac.tokens;
                                    let macro_str = quote!(#macro_tokens).to_string();
                                    let new_macro_str = re
                                        .replace_all(&macro_str, |caps: &Captures| {
                                            format!("*self._{}()", &caps[1])
                                        })
                                        .to_string();
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
                                    // way1: use syn
                                    // 遍历函数调用的参数
                                    let mut new_args = Vec::new();
                                    for arg in &expr_call.args {
                                        if let syn::Expr::Reference(expr_ref) = arg {
                                            // 检查是否是 `&mut self.<field>` 形式
                                            if expr_ref.mutability.is_some() {
                                                // 如果有 `mut` 关键字
                                                if let syn::Expr::Path(expr_path) = &*expr_ref.expr
                                                {
                                                    if expr_path.path.is_ident("sxxxelf") {
                                                        // 替换为 `self._<field>_mut()`
                                                        let field_name = expr_path
                                                            .path
                                                            .segments
                                                            .last()
                                                            .unwrap()
                                                            .ident
                                                            .to_string();
                                                        let new_arg = syn::parse_str::<syn::Expr>(
                                                            &format!("self._{}_mut()", field_name),
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
                                        let expr_str = quote!(#expr).to_string();
                                        let new_expr_str = re
                                            .replace_all(&expr_str, |caps: &Captures| {
                                                format!("self._{}()", &caps[1])
                                            })
                                            .to_string();
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
                            let new_expr: syn::Expr =
                                syn::parse_str(&new_expr_str).expect("Failed to parse new expr");
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
                            let new_stmt: syn::Stmt =
                                syn::parse_str(&new_stmt_str).expect("Failed to parse new stmt");
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
            quote! { #method }
        } else {
            quote! { #item }
        }
    });

    // 4. generate the hidden declarative macro for target struct
    let decl_macro_code = quote! {
        #[doc(hidden)]
        #[macro_export] // it is ok to always export the declarative macro
        macro_rules! #trait_decl_macro_name { // NOTE: the reexpanded macro is used for rust struct only
            (
                ($hidden_parent_trait:path)
                $(#[$struct_attr:meta])* // NOTE: make sure the style is consistent with that in arm 2 output
                $vis:vis struct $struct_name:ident {
                    $($struct_content:tt)*
                }
            ) => {
                $(#[$struct_attr])*
                $vis struct $struct_name {
                    $($struct_content)*
                    #(
                        #struct_trait_fields_defs
                    )*
                }
                impl $hidden_parent_trait for $struct_name {
                    #(
                        #parent_trait_methods_impls
                    )*
                }
            };
        }
    };
    // 5. expand the final code
    let expanded = quote! {
        #trait_vis trait #parent_trait_name {
            #(#parent_trait_methods)*
        }
        #trait_vis trait #trait_name: #parent_trait_name {
            #(#original_trait_items)*
        }

        #decl_macro_code
    };
    TokenStream::from(expanded)
}

/// attribute macro: used to tag Rust struct like: `#[trait_var(<trait_name>)]`
#[proc_macro_attribute]
pub fn trait_var(args: TokenStream, input: TokenStream) -> TokenStream {
    // parse attributes
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let trait_name = match args.first().unwrap() {
        syn::NestedMeta::Meta(syn::Meta::Path(path)) => path.get_ident().unwrap(),
        _ => panic!("Expected a trait name"),
    };

    // parse input, only accept `struct`
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let visible = &input_struct.vis;
    let struct_name = &input_struct.ident;

    // handle different visibility of the struct fields
    let struct_fields = input_struct.fields.iter().map(|f| {
        let field_vis = &f.vis;
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        quote! {
            #field_vis #field_ident: #field_ty,
        }
    });

    // expand code
    let trait_macro_name = Ident::new(&format!("{}_for_struct", trait_name), trait_name.span());
    let parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    let expanded = quote! {
        #trait_macro_name! {
            (#parent_trait_name)
            // (#hidden_trait_path) // TODO: delete?
            #visible struct #struct_name {
                #(#struct_fields)*
            }
        }
    };

    // return
    expanded.into()
}
