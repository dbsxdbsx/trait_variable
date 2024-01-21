extern crate proc_macro;

// use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, ItemTrait, TraitItem, TraitItemMethod};

use proc_macro::{Ident, Span, TokenStream};

#[proc_macro]
pub fn echo(input: TokenStream) -> TokenStream {
    input
}


// #[proc_macro_attribute]
// pub fn trait_var(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let attr_str = attr.to_string();
//     let trait_name = parse_macro_input!(attr as Ident);
//     let input = parse_macro_input!(item as DeriveInput);
//     let struct_name = &input.ident;

//     // 解析特性定义，以便我们可以检查其中的方法
//     let trait_ast: ItemTrait = syn::parse_str(&attr_str).expect("Failed to parse trait");

//     // 遍历特性中的所有项目
//     let methods = trait_ast.items.into_iter().filter_map(|item| {
//         if let TraitItem::Method(TraitItemMethod { sig, .. }) = item {
//             // 检查方法名称是否符合特定格式
//             let method_name = &sig.ident;
//             let method_name_str = method_name.to_string();
//             if let Some(stripped) = method_name_str.strip_prefix('_') {
//                 // 提取类型名称和字段名称
//                 let type_name = &sig.output;
//                 let field_name = format_ident!("{}", stripped);
//                 // 生成对应的方法实现
//                 let generated = quote! {
//                     fn #method_name(&self) -> #type_name {
//                         &self.#field_name
//                     }
//                 };
//                 return Some(generated);
//             }
//         }
//         None
//     });

//     // 生成最终的impl块
//     let gen = quote! {
//         impl #trait_name for #struct_name {
//             #(#methods)*
//         }
//     };

//     gen.into()
// }
