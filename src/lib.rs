extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, parse_quote, DeriveInput, ItemTrait, TraitItem, TraitItemMethod};

#[proc_macro_attribute]
pub fn trait_var(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let trait_name = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    // 解析特性定义，以便我们可以检查其中的方法
    let trait_ast: ItemTrait = syn::parse_str(&attr_str).expect("Failed to parse trait");

    // 遍历特性中的所有项目
    let methods = trait_ast.items.into_iter().filter_map(|item| {
        if let TraitItem::Method(TraitItemMethod { sig, .. }) = item {
            // 检查方法名称是否符合特定格式
            let method_name = &sig.ident;
            let method_name_str = method_name.to_string();
            if method_name_str.starts_with('_') {
                // 提取类型名称和字段名称
                let type_name = &sig.output;
                let field_name = format_ident!("{}", &method_name_str[1..]);
                // 生成对应的方法实现
                let generated = quote! {
                    fn #method_name(&self) -> #type_name {
                        &self.#field_name
                    }
                };
                return Some(generated);
            }
        }
        None
    });

    // 生成最终的impl块
    let gen = quote! {
        impl #trait_name for #name {
            #(#methods)*
        }
    };

    gen.into()
}
