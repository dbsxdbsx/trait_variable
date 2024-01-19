use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemStruct, ItemTrait};
use syn::{parse_str, TraitItem};

pub fn modify_struct(attr_name: &Ident, struct_item: &ItemStruct) -> TokenStream {
    println!("attr_name for struct: {}", attr_name.to_string());
    assert!(
        !attr_name.to_string().is_empty(),
        "attr_name should not be empty for struct"
    );

    // TODO: 执行后续逻辑
    let ts: TokenStream = quote!(#struct_item).into();
    println!("haha: {}", ts.to_string());
    ts
}
