extern crate proc_macro;
mod trait_item;
mod utils;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use syn::parse::discouraged::Speculative;
use syn::{
    braced, token, Generics, PathArguments, PathSegment, TypeParamBound, TypePath, Visibility,
    WhereClause,
};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, TraitItem, Type,
};
use trait_item::refine_trait_items;

struct TraitVarField {
    var_vis: Visibility,
    var_name: Ident,
    _colon_token: Token![:],
    type_name: syn::Type, // Changed from TokenStream to syn::Type
    is_generic_type: bool,
    _semicolon_token: Token![;],
}

impl Parse for TraitVarField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let var_vis: Visibility = input.parse()?;
        let var_name: Ident = input.parse()?;
        let _colon_token: Token![:] = input.parse()?;
        let type_name: syn::Type = input.parse()?; // Adjusted to parse into syn::Type
        let is_generic_type = type_name.to_token_stream().to_string().len() == 1
            && type_name
                .to_token_stream()
                .to_string()
                .chars()
                .next()
                .unwrap()
                .is_uppercase();
        let _semicolon_token: Token![;] = input.parse()?;

        Ok(TraitVarField {
            var_vis,
            var_name,
            _colon_token,
            type_name,
            is_generic_type,
            _semicolon_token,
        })
    }
}

struct TraitInput {
    trait_vis: Visibility,
    _trait_token: Token![trait],
    trait_name: Ident,
    trait_bounds: Option<Generics>, // optional generic parameters for the trait
    parent_traits: Option<TypeParamBound>, // optional parent trait bound
    where_clause: Option<WhereClause>, // optional where clause for the trait
    _brace_token: token::Brace,
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
            parent_traits: if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                Some(input.parse()?) // Parse the parent trait bound
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
                let mut vars = Vec::new();
                while !content.peek(Token![type])
                    && !content.peek(Token![const])
                    && !content.peek(Token![fn])
                    // && !content.peek(Token![;]) // TODO: delete?
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
        parent_traits,
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
    let parent_trait_methods_signatures = trait_variables.iter().map(
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
        let generic_types: Vec<_> = trait_variables
            .iter()
            .filter(|field| field.is_generic_type)
            .map(|field| &field.type_name)
            .collect();
        if !generic_types.is_empty() {
            let generics_list = quote! { <#(#generic_types),*> };
            generics_list.into_token_stream()
        } else {
            TokenStream::new()
        }
    };

    // 3. refine the body of methods from the original trait
    let trait_items = refine_trait_items(trait_items);

    // 4. generate the hidden declarative macro for target struct
    let decl_macro_code = quote! {
        #[doc(hidden)]
        #[macro_export] // it is ok to always export the declarative macro
        macro_rules! #trait_decl_macro_name { // NOTE: the reexpanded macro is used for rust struct onl
            (
                ($hidden_parent_trait:path)
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
                $hidden_parent_trait #hidden_parent_trait_bounds
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

    // 5. expand the final code
    let expanded = quote! {
        #trait_vis trait #hidden_parent_trait_name #hidden_parent_trait_bounds {
            #(#parent_trait_methods_signatures)*
        }
        #trait_vis trait #trait_name #trait_bounds: #hidden_parent_trait_name + #parent_traits #where_clause {
            #(#trait_items)*
        }

        #decl_macro_code
    };
    proc_macro::TokenStream::from(expanded)
}

/// attribute macro: used to tag Rust struct like: `#[trait_var(<trait_name>)]`
#[proc_macro_attribute]
pub fn trait_var(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
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
    let hidden_parent_trait_name = Ident::new(&format!("_{}", trait_name), trait_name.span());
    let expanded = quote! {
        #trait_macro_name! {
            (#hidden_parent_trait_name)
            // (#hidden_trait_path) // TODO: delete?
            #visible struct #struct_name #generics {
                #(#original_struct_fields)*
            }
        }
    };

    // return
    expanded.into()
}
