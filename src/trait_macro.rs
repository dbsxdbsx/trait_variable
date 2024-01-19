// #[macro_export] --- can't export macro from proc-macro crate

macro_rules! trait_var {
    (
        trait $trait_name:ident {
            $(let $var_name:ident : $var_type:ty;)*
            $(fn $fn_name:ident(&self);)*
        }
    ) => {
        trait $trait_name {
            $(
                fn $var_name(&self) -> &$var_type;
                fn concat_idents!($var_name, _mut)(&mut self) -> &mut $var_type;
            )*
            $(
                fn $fn_name(&self);
            )*
        }
    };
}

// use proc_macro::TokenStream;
// use quote::{quote, ToTokens};
// use syn::Token;
// use syn::{
//     parse::Parse, parse::ParseStream, parse_macro_input, Fields, Ident, Item, ItemTrait, Result,
//     TraitItem, Type,
// };

// use proc_macro2::TokenStream as TokenStream2;

// pub fn modify_trait(attr_name: &Ident, trait_item: &ItemTrait) -> TokenStream {
//     assert!(
//         attr_name.to_string().is_empty(),
//         "attr_name should be empty for trait"
//     );

//     let mut new_items: Vec<TokenStream2> = Vec::new();

//     for item in &trait_item.items {
//         if let TraitItem::Verbatim(verbatim) = item {
//             if let Ok(let_statement) = syn::parse2::<LetStatement>(verbatim.into_token_stream()) {
//                 let field_name = &let_statement.field_name;
//                 let field_type = &let_statement.field_type;
//                 let getter = quote! {
//                     fn #field_name(&self) -> &#field_type;
//                 };
//                 let setter = quote! {
//                     fn #field_name(&mut self, value: #field_type);
//                 };
//                 new_items.push(getter);
//                 new_items.push(setter);
//             } else {
//                 new_items.push(quote! { #verbatim });
//             }
//         } else {
//             new_items.push(quote! { #item });
//         }
//     }

//     let ts: TokenStream2 = quote! {
//         #(#new_items)*
//     };

//     println!("the final trait code:\n{}", ts.to_string());

//     ts.into()
// }

// struct LetStatement {
//     let_token: Token![let],
//     field_name: Ident,
//     colon_token: Token![:],
//     field_type: Type,
//     semi_token: Token![;],
// }

// impl Parse for LetStatement {
//     fn parse(input: ParseStream) -> Result<Self> {
//         let let_token = input.parse()?;
//         let field_name = input.parse()?;
//         let colon_token = input.parse()?;
//         let field_type = input.parse()?;
//         let semi_token = input.parse()?;
//         Ok(LetStatement {
//             let_token,
//             field_name,
//             colon_token,
//             field_type,
//             semi_token,
//         })
//     }
// }
