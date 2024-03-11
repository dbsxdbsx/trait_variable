extern crate proc_macro;
mod trait_item;
mod utils;

use proc_macro::TokenStream;
use quote::quote;

use syn::{braced, token, Generics, Visibility, WhereClause};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Ident, Token, TraitItem, Type,
};
use trait_item::refine_trait_items;

/// The struct used to support `pub var:i32;` like trait variable declaration
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
    trait_bound: Option<Generics>, // optional generic parameters for the trait
    where_clause: Option<WhereClause>, // optional where clause for the trait
    _brace_token: token::Brace,
    trait_variables: Punctuated<TraitVarField, Token![;]>,
    trait_items: Vec<TraitItem>, // all valid trait items, including methods, constants, and associated types
}

impl Parse for TraitInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;

        Ok(TraitInput {
            trait_vis: input.parse()?,
            _trait_token: input.parse()?,
            trait_name: input.parse()?,
            trait_bound: if input.peek(Token![<]) {
                Some(input.parse()?) // Use the parse method to parse the generics
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
                let mut vars = Punctuated::new();
                while !content.peek(Token![type])
                    && !content.peek(Token![const])
                    && !content.peek(Token![fn])
                    && !content.peek(Token![;])
                    && !content.is_empty()
                {
                    vars.push_value(content.parse()?);
                    // Ensure that a semicolon follows the variable declaration
                    if !content.peek(Token![;]) {
                        return Err(content.error("expected `;` after variable declaration"));
                    }
                    vars.push_punct(content.parse()?);
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
pub fn trait_variable(input: TokenStream) -> TokenStream {
    let TraitInput {
        trait_vis,
        trait_name,
        trait_bound,
        where_clause,
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

    // 3.1 Extract the generics, if present
    // let trait_bound = trait_bound.unwrap_or_else(|| Generics::default());
    // let (_, trait_bound, _) = trait_bound.split_for_impl();
    let trait_bound = trait_bound.clone();
    let where_clause = where_clause.clone();

    // 3.2 refine the body of methods from the original trait
    let trait_items = refine_trait_items(trait_items);

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
        #trait_vis trait #trait_name #trait_bound: #parent_trait_name #where_clause{
            #(#trait_items)*
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
