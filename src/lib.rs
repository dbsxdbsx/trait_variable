use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, ItemStruct, ItemTrait};

// TODO: 处理trait的函数签名
fn handle_trait(attr_name: &Ident, trait_item: &ItemTrait) -> TokenStream {
    assert!(
        attr_name.to_string().is_empty(),
        "attr_name should be empty for trait"
    );

    // Implement the logic to handle the trait
    let ts: TokenStream = quote!().into();
    println!("{}", ts.to_string());
    ts
}

// TODO: 处理struct的函数签名
fn handle_struct(attr_name: &Ident, struct_item: &ItemStruct) -> TokenStream {
    assert!(
        !attr_name.to_string().is_empty(),
        "attr_name should not be empty for struct"
    );

    // Implement the logic to handle the struct
    let ts: TokenStream = quote!().into();
    println!("{}", ts.to_string());
    ts
}

#[proc_macro_attribute]
pub fn trait_variable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_name = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(item as Item);

    match input {
        Item::Trait(trait_item) => handle_trait(&attr_name, &trait_item),
        Item::Struct(struct_item) => handle_struct(&attr_name, &struct_item),
        _ => {
            panic!("`#[trait_variable]` can only be used on traits and structs");
        }
    }
}
