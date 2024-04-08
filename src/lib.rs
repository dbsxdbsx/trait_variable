mod trait_item;
mod utils;

use proc_macro2::TokenStream;

use quote::{quote, ToTokens};

use syn::visit::{self, Visit};
use syn::{
    braced, token, AngleBracketedGenericArguments, GenericArgument, Generics, PathArguments, Type,
    TypeParamBound, TypePath, Visibility, WhereClause,
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
impl GenericTypeVisitor {
    fn is_single_upper_letter(&self, ident_str: &str) -> bool {
        ident_str.len() == 1 && ident_str.chars().next().unwrap().is_uppercase()
    }
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
                            if self.is_single_upper_letter(&ident_str)
                                && !self.generics.contains(&ident_str)
                            {
                                self.generics.push(ident_str);
                            }
                        }
                    }
                }
            } else if let Some(seg) = path.segments.last() {
                let ident_str = seg.ident.to_string();
                if self.is_single_upper_letter(&ident_str) && !self.generics.contains(&ident_str) {
                    self.generics.push(ident_str);
                }
            }
        }
        // Continue the traversal to nested types
        visit::visit_type(self, i);
    }
}
#[test]
fn test_generic_type_visitor() {
    // case 1
    let code = quote! { V }; // the quoted type is invalid, but ok for test
    let syntax_tree: syn::Type = syn::parse2(code).unwrap();
    let mut visitor = GenericTypeVisitor {
        generics: Vec::new(),
    };
    visitor.visit_type(&syntax_tree);

    assert_eq!(visitor.generics, vec!["V"]);
    // case 2
    let code = quote! { Vec<T, HashMap<K, V>> }; // the quoted type is invalid, but ok for test
    let syntax_tree: syn::Type = syn::parse2(code).unwrap();
    let mut visitor = GenericTypeVisitor {
        generics: Vec::new(),
    };
    visitor.visit_type(&syntax_tree);

    assert_eq!(visitor.generics, vec!["T", "K", "V"]);
}

/// Define the struct to represent a single trait variable field.
struct TraitVarField {
    var_vis: Visibility,
    var_name: Ident,
    type_name: Type,
    type_generics: Vec<String>,
}
impl Parse for TraitVarField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let var_vis: Visibility = input.parse().expect("Failed to Parse to `var_vis`");
        let var_name: Ident = input.parse().expect("Failed to Parse to `var_name`");
        let _: Token![:] = input.parse().expect("Failed to Parse to `:`");
        let type_name: Type = input.parse().expect("Failed to Parse to `type_name`");
        let type_generics = {
            let mut visitor = GenericTypeVisitor {
                generics: Vec::new(),
            };
            visitor.visit_type(&type_name);
            visitor.generics
        };
        Ok(TraitVarField {
            var_vis,
            var_name,
            type_name,
            type_generics,
        })
    }
}
#[test]
fn test_trait_var_field() {
    let raw_code = quote! { pub var_name: Vec<T, HashMap<K, V>> };
    let parsed =
        syn::parse2::<TraitVarField>(raw_code).expect("Failed to parse to `TraitVarField`");

    assert!(
        matches!(parsed.var_vis, Visibility::Public(_)),
        "Visibility is not public"
    );
    assert_eq!(parsed.var_name.to_string(), "var_name".to_string());
    assert_eq!(
        parsed.type_name.to_token_stream().to_string(),
        "Vec < T , HashMap < K , V > >".to_string()
    );
    assert_eq!(
        parsed.type_generics,
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
    trait_variables: Vec<TraitVarField>,
    trait_items: Vec<TraitItem>,
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
                let mut v = Vec::new();
                while !content.peek(Token![type])
                    && !content.peek(Token![const])
                    && !content.peek(Token![fn])
                    && !content.is_empty()
                {
                    v.push(content.call(TraitVarField::parse)?);
                    let _: Token![;] = content.parse()?;
                }
                v
            },
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

#[test]
fn test_trait_input() {
    let raw_code = quote! {
        pub trait MyTrait {
            x: Vec<T, HashMap<K, V>>;
            pub y: bool;

            fn print_x(&self){
                println!("x: `{}`", self.x);
            }
            fn print_y(&self){
                println!("y: `{}`", self.y);
            }
            fn print_all(&self);
        }
    };
    let parsed = syn::parse2::<TraitInput>(raw_code).unwrap();

    assert!(matches!(parsed.trait_vis, Visibility::Public(_)));
    assert_eq!(parsed.trait_name.to_string(), "MyTrait".to_string());
    assert!(parsed.trait_bounds.is_none());
    assert!(parsed.explicit_parent_traits.is_none());
    assert!(parsed.where_clause.is_none());
    assert_eq!(parsed.trait_variables.len(), 2);
    assert_eq!(
        parsed.trait_variables[0].var_name.to_string(),
        "x".to_string()
    );
    assert_eq!(
        parsed.trait_variables[1].var_name.to_string(),
        "y".to_string()
    );
    assert_eq!(parsed.trait_items.len(), 3);
    assert_eq!(
        parsed.trait_items[0].to_token_stream().to_string(),
        "fn print_x (& self) { println ! (\"x: `{}`\" , self . x) ; }".to_string()
    );
    assert_eq!(
        parsed.trait_items[1].to_token_stream().to_string(),
        "fn print_y (& self) { println ! (\"y: `{}`\" , self . y) ; }".to_string()
    );
    assert_eq!(
        parsed.trait_items[2].to_token_stream().to_string(),
        "fn print_all (& self) ;".to_string()
    );
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
            for generic in &trait_var.type_generics {
                let generic_ident = syn::Ident::new(generic, proc_macro2::Span::call_site());
                if !generic_types.contains(&generic_ident) {
                    generic_types.push(generic_ident);
                }
            }
        }
        if !generic_types.is_empty() {
            quote! { <#(#generic_types),*> }
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
                $(where $($where_clause:tt)*)?
                {
                    $($struct_content:tt)*
                }
            ) => {
                // 1. the struct definition block with trait variable fields:
                $(#[$struct_attr])*
                $vis struct $struct_name
                $(<$($generic_param),* $(, $generic_lifetime)*>)?
                $(where $($where_clause)*)?
                {
                    $($struct_content)*
                    #(
                        #trait_fields_in_struct
                    )*
                }
                // 2. the struct impl block for the hidden parent trait:
                impl
                // 2.1 the struct generic+lifetime parameters, if any
                $(<$($generic_param),* $(, $generic_lifetime)*>)?
                // 2.2 the hidden parent trait
                #hidden_parent_trait_with_bounds
                for
                // 2.3 the struct name with generic parameters, if any
                $struct_name
                $(<$($generic_param),* $(, $generic_lifetime)*>)?
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
    // Convert TokenStream to ParseStream
    let AttrArgs(trait_name) = parse_macro_input!(args as AttrArgs);

    // parse input, only accept `struct`
    let input_struct = parse_macro_input!(input as syn::ItemStruct);
    let visible = &input_struct.vis;
    let struct_name = &input_struct.ident;
    let generics = &input_struct.generics;

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
