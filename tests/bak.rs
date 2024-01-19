// #[cfg(test)]
// mod tests {
//     use super::trait_var2;
//     use proc_macro2::TokenStream;
//     use quote::quote;

//     #[test]
//     fn test_trait_var2() {
//         let attr: TokenStream = quote! { MyTrait };
//         let item: TokenStream = quote! {
//             struct MyStruct {
//                 field: i32,
//             }
//         };

//         let output = trait_var2(attr, item);
//         // 这里你可以对output进行断言，检查它是否符合你的期望
//     }
// }
