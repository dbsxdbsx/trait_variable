mod trait_item;
mod utils;

use std::collections::HashSet;

use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::{quote, ToTokens};

use syn::visit::{self, Visit};
use syn::{
    braced, parse2, token, AngleBracketedGenericArguments, Attribute, DeriveInput, GenericArgument,
    GenericParam, Generics, PathArguments, Type, TypeParamBound, TypePath, Visibility, WhereClause,
};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, TraitItem,
};
use trait_item::refine_trait_items;

struct GenericTypeVisitor {
    generics: Vec<String>,
}
impl<'ast> Visit<'ast> for GenericTypeVisitor {
    fn visit_type(&mut self, i: &'ast Type) {
        if let Type::Path(TypePath { path, .. }) = i {
            if let Some(PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                args,
                ..
            })) = path.segments.last().map(|seg| &seg.arguments)
            {
                for arg in args {
                    if let GenericArgument::Type(Type::Path(tp)) = arg {
                        if let Some(ident) = tp.path.get_ident() {
                            let ident_str = ident.to_string();
                            if ident_str.len() == 1
                                && ident_str.chars().next().unwrap().is_uppercase()
                            {
                                self.generics.push(ident_str);
                            }
                        }
                    }
                }
            }
        }
        // Continue the traversal to nested types
        visit::visit_type(self, i);
    }
}
#[test]
fn test_generic_type_visitor() {
    let code = quote! { Vec<T, HashMap<K, V>> }; // the quoted type is invalid, but ok for test
    let syntax_tree: syn::Type = syn::parse2(code).unwrap();
    let mut visitor = GenericTypeVisitor {
        generics: Vec::new(),
    };
    visitor.visit_type(&syntax_tree);

    assert_eq!(visitor.generics, vec!["T", "K", "V"]);
}

/// Define the enum to represent different kinds of trait variable types.
/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
struct TraitVarType {
    name: TokenStream, // the whole type name, including generics, like `HashMap<K, V>`, `i32`, `T`, etc.
    generics: Vec<String>, // the generic type elements in the trait type, like `K, V` in `HashMap<K, V>`
}
impl ToTokens for TraitVarType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.name.clone());
    }
}
impl Parse for TraitVarType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = TokenStream::new();
        let mut generics = Vec::new();

        // 1. Parse until a semicolon is found, indicating the end of a type definition
        while !input.is_empty() {
            if input.peek(Token![;]) {
                println!("the EXIT token is: {}", input.parse::<TokenTree>()?);
                break;
            }
            let token = input.parse::<TokenTree>()?;
            println!("the token is:{}", token.to_string());
            name.extend(Some(token));
        }

        println!("finally， the name is:{}", name.to_string());

        // 2. Parse generics
        if let Ok(type_parsed) = syn::parse2::<Type>(name.clone().into()) {
            let mut visitor = GenericTypeVisitor {
                generics: Vec::new(),
            };
            visitor.visit_type(&type_parsed);
            generics.extend(visitor.generics);
        }

        // 3. Return
        Ok(TraitVarType { name, generics })
    }
}

#[test]
fn test_trait_var_type() {
    let raw_code = quote! { Vec<T, HashMap<K, V>>;};
    println!("the raw code is:`{}`", raw_code);
    let parsed = parse2::<TraitVarType>(raw_code.clone()).expect("Failed to parse");
    println!("the raw code is:`{}`", raw_code);

    assert_eq!(
        parsed.name.to_string(),
        "Vec < T , HashMap < K , V >>".to_string()
    );
    assert_eq!(
        parsed.generics,
        vec!["T".to_string(), "K".to_string(), "V".to_string()]
    );
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/// Define the struct to represent a single trait variable field.
struct TraitVarField {
    var_vis: Visibility,
    var_name: Ident,
    type_name: TraitVarType,
}
impl Parse for TraitVarField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("the orig input is:{}", input);
        let var_vis: Visibility = input.parse().expect("Failed to Parse to `var_vis`");
        println!("the input is:{}", input);
        let var_name: Ident = input.parse().expect("Failed to Parse to `var_name`");
        println!("the input is:{}", input);
        let _: Token![:] = input.parse().expect("Failed to Parse to `:`");
        println!("the input before TraitVarType is:{}", input);
        let type_name: TraitVarType = input.parse().expect("Failed to Parse to `type_name`");
        println!("the input after TraitVarType is:{}", input);
        let _: Token![;] = input.parse().expect("Failed to Parse to `;`");

        Ok(TraitVarField {
            var_vis,
            var_name,
            type_name,
        })
    }
}
#[test]
fn test_trait_var_field() {
    let raw_code = quote! { pub var_name: Vec<T, HashMap<K, V>>; };
    let parsed = parse2::<TraitVarField>(raw_code).expect("Failed to parse to `TraitVarField`");

    assert!(
        matches!(parsed.var_vis, Visibility::Public(_)),
        "Visibility is not public"
    );
    assert_eq!(parsed.var_name.to_string(), "var_name".to_string());
    assert_eq!(
        parsed.type_name.name.to_string(),
        "Vec < T , HashMap < K , V >>".to_string()
    );
    assert_eq!(
        parsed.type_name.generics,
        vec!["T".to_string(), "K".to_string(), "V".to_string()]
    );
}

struct TraitInput {
    trait_vis: Visibility,
    _trait_token: Token![trait],
    trait_name: Ident,
    trait_bounds: Option<Generics>, // optional generic parameters for the trait
    explicit_parent_traits: Option<Punctuated<TypeParamBound, Token![+]>>, // explicit parent traits
    where_clause: Option<WhereClause>, // optional where clause for the trait
    _brace_token: token::Brace,
    // trait_variables: Punctuated<TraitVarField, Token![;]>,// TODO: delete
    trait_variables: Vec<TraitVarField>,
    trait_items: Vec<TraitItem>, // all valid trait items, including methods, constants, and associated types
}

impl Parse for TraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(TraitInput {
            trait_vis: input.parse()?,
            _trait_token: input.parse()?,
            trait_name: input.parse()?,
            trait_bounds: if input.peek(Token![<]) {
                Some(input.parse()?) // Use the parse method to parse the generics
            } else {
                None
            },
            explicit_parent_traits: if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                let mut parent_traits = Punctuated::new();
                while !input.peek(Token![where]) && !input.peek(token::Brace) {
                    parent_traits.push_value(input.parse()?);
                    if input.peek(Token![+]) {
                        parent_traits.push_punct(input.parse()?);
                    } else {
                        break;
                    }
                }
                Some(parent_traits)
            } else {
                None
            },
            where_clause: if input.peek(syn::token::Where) {
                Some(input.parse()?)
            } else {
                None
            },
            _brace_token: braced!(content in input),
            // Parse all variable declarations until a method or end of input is encountered
            trait_variables: {
                // TODO: delete
                // if !content.peek(Token![type])
                //     && !content.peek(Token![const])
                //     && !content.peek(Token![fn])
                //     && !content.is_empty()
                // {
                //     content.parse_terminated(TraitVarField::parse, Token![;])?
                // } else {
                //     Punctuated::new()
                // }
                let mut vars = Vec::new();
                while !content.peek(Token![type])
                    && !content.peek(Token![const])
                    && !content.peek(Token![fn])
                    && !content.is_empty()
                {
                    vars.push(content.parse()?);
                }
                vars
            },
            // Parse all valid trait items, including methods, constants, and associated types
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
pub fn trait_variable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let TraitInput {
        trait_vis,
        trait_name,
        trait_bounds,
        explicit_parent_traits,
        where_clause,
        trait_variables,
        trait_items,
        ..
    } = parse_macro_input!(input as TraitInput);

    // 1.1 get parent trait name
    let hidden_parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    // 1.2 get trait declarative macro name
    let trait_decl_macro_name =
        Ident::new(&format!("{}_for_struct", trait_name), trait_name.span());

    // 2.1 generate parent trait methods declaration
    let hidden_parent_trait_methods_signatures = trait_variables.iter().map(
        |TraitVarField {
             var_name,
             type_name,
             ..
         }| {
            let method_name = Ident::new(&format!("_{}", var_name), var_name.span());
            let method_name_mut = Ident::new(&format!("_{}_mut", var_name), var_name.span());
            quote! {
                fn #method_name(&self) -> &#type_name;
                fn #method_name_mut(&mut self) -> &mut #type_name;
            }
        },
    );
    // 2.2 generate trait variable fields definition for structs later
    let trait_fields_in_struct = trait_variables.iter().map(
        |TraitVarField {
             var_vis,
             var_name,
             type_name,
             ..
         }| {
            quote! {
                #var_vis #var_name: #type_name,
            }
        },
    );
    // 2.3 generate parent trait methods implementation for struct
    let parent_trait_methods_impls_in_struct = trait_variables.iter().map(
        |TraitVarField {
             var_name,
             type_name,
             ..
         }| {
            let method_name = Ident::new(&format!("_{}", var_name), var_name.span());
            let method_name_mut = Ident::new(&format!("_{}_mut", var_name), var_name.span());
            quote! {
                fn #method_name(&self) -> &#type_name{
                    &self.#var_name
                }
                fn #method_name_mut(&mut self) -> &mut #type_name{
                    &mut self.#var_name
                }
            }
        },
    );
    // 2.4 check if the parent trait has generic type
    let hidden_parent_trait_bounds = {
        let mut generic_types = Vec::new();
        for trait_var in trait_variables.iter() {
            for generic in &trait_var.type_name.generics {
                if generic_types.contains(generic) {
                    continue;
                }
                generic_types.push(generic.clone());
            }
        }

        if !generic_types.is_empty() {
            let generics_list = quote! { <#(#generic_types),*> };
            generics_list.into_token_stream()
        } else {
            TokenStream::new()
        }
    };

    // 3. refine the body of methods from the original trait
    let trait_items = refine_trait_items(trait_items);

    // 4. expand the trait code
    let hidden_parent_trait_with_bounds =
        quote! {#hidden_parent_trait_name #hidden_parent_trait_bounds};
    let expanded_trait_code = quote! {
        #trait_vis trait #hidden_parent_trait_with_bounds {
            #(#hidden_parent_trait_methods_signatures)*
        }
        #trait_vis trait #trait_name #trait_bounds: #hidden_parent_trait_with_bounds + #explicit_parent_traits #where_clause {
            #(#trait_items)*
        }
    };

    // 5. generate the hidden declarative macro for target struct
    let declarative_macro_code = quote! {
        #[doc(hidden)]
        #[macro_export] // it is ok to always export the declarative macro
        macro_rules! #trait_decl_macro_name { // NOTE: the reexpanded macro is used for rust struct onl
            (
                $(#[$struct_attr:meta])* // NOTE: make sure the style is consistent with that in arm 2 output
                $vis:vis struct $struct_name:ident
                $(<$($generic_param:ident),* $(, $generic_lifetime:lifetime)* $(,)? >)?
                // $(where $($where_clause:tt)+)? // TODO
                {
                    $($struct_content:tt)*
                }
            ) => {
                // 1. the struct definition:
                $(#[$struct_attr])*
                $vis struct $struct_name
                $(<$($generic_param),* $(, $generic_lifetime)*>)?
                // TODO: $(where $($where_clause)+)?
                {
                    $($struct_content)*
                    #(
                        #trait_fields_in_struct
                    )*
                }
                // 2. the struct impl block:
                impl
                // 2.1 the struct generic+lifetime parameters, if any
                $(<$($generic_param),* $(, $generic_lifetime)*>)?
                // 2.2 the hidden parent trait
                #hidden_parent_trait_with_bounds
                for
                // 2.3 the struct name with generic parameters, if any
                $struct_name
                $(<$($generic_param),* $(, $generic_lifetime)*>)?
                // TODO: $(where $($where_clause)+)?
                {
                    #(
                        #parent_trait_methods_impls_in_struct
                    )*
                }
            };
        }
    };

    // 6. integrate all expanded code
    proc_macro::TokenStream::from(quote! {
        #expanded_trait_code
        #declarative_macro_code
    })
}

struct AttrArgs(Ident);

impl syn::parse::Parse for AttrArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        Ok(AttrArgs(ident))
    }
}
/// attribute macro: used to tag Rust struct like: `#[trait_var(<trait_name>)]`
#[proc_macro_attribute]
pub fn trait_var(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // 将 TokenStream 转换为 ParseStream
    let AttrArgs(trait_name) = parse_macro_input!(args as AttrArgs);

    // parse input, only accept `struct`
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let visible = &input_struct.vis;
    let struct_name = &input_struct.ident;
    let generics = &input_struct.generics; // parse generics

    // handle different visibility of the struct fields
    // NOTE: the `original_struct_fields` does not include the hidden trait variable fields
    let original_struct_fields = input_struct.fields.iter().map(|f| {
        let field_vis = &f.vis;
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        quote! {
            #field_vis #field_ident: #field_ty,
        }
    });

    // expand code
    let trait_macro_name = Ident::new(&format!("{}_for_struct", trait_name), trait_name.span());
    let _hidden_parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    let expanded = quote! {
        #trait_macro_name! {
            // (#hidden_trait_path) // TODO: delete?
            #visible struct #struct_name #generics {
                #(#original_struct_fields)*
            }
        }
    };

    // return
    expanded.into()
}
