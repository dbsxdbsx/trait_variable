use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemStruct, ItemTrait};
mod cache;
mod struct_macro;
mod trait_macro; // 新增的缓存模块

#[proc_macro_attribute]
pub fn trait_variable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_name = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(item as Item);

    match input {
        Item::Trait(trait_item) => {
            // cache::cache_trait(&attr_name, &trait_item); //TODO: 将特性存储到缓存中
            trait_macro::modify_trait(&attr_name, &trait_item)
        }
        Item::Struct(struct_item) => {
            // let trait_item = cache::get_trait(&attr_name); // TODO: 从缓存中获取特性
            // struct_macro::modify_struct(&attr_name, &struct_item, &trait_item) // 修改函数，增加一个参数
            struct_macro::modify_struct(&attr_name, &struct_item)
        }
        _ => {
            panic!("`#[trait_variable]` can only be used on traits and structs");
        }
    }
}


