pub trait TraitEnhanceType<'a> {
    type View;
    type ViewMut;
}

pub trait TraitEnhance: for<'a> TraitEnhanceType<'a> {
    fn get_fields(&self) -> <Self as TraitEnhanceType<'_>>::View;
    fn get_fields_mut(&mut self) -> <Self as TraitEnhanceType<'_>>::ViewMut;
}

#[macro_export]
macro_rules! trait_variable {
    // 1. Entry point for parsing a trait:
    (
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident {
            $($trait_content:tt)*
        }
    ) => {
        $crate::trait_variable!{
            @enhance_trait  // NOTE: this is a recursive call
            trait_def = {
                $(#[$attr])*
                $vis trait $trait_name
            },
            content = { $($trait_content)* },
            fields = {},
            dollar = {$},
        }
    };
    // 2. Entry point for parsing a struct, to generated macro next to the trait:
    (
        #[trait_var($trait_name:path)] // this line is just used as a tag
        // ($trait:path) // this line is just used as a tag
        $(#[$attr:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_name:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        paste::paste!{
            [<$trait_name _for_struct>] !{ // NOTE: this is the expanded macro from arm 2
                $(#[$attr])*
                $vis struct $struct_name {
                    $(
                        $(#[$field_attr:meta])*
                        $field_vis $field_name : $field_type,
                    )*
                }
            }
        }
    };
    // 3.Parsing trait (has more fields):
    (@enhance_trait
        trait_def = $trait_def:tt,
        content = {
            $(#[$field_attr:meta])*
            let $field_name:ident: $field_type:ty;
            $($trait_content:tt)*
        },
        fields = { $($prev_fields:tt)* },
        dollar = {$dollar:tt},
    ) => {
        $crate::trait_variable! {
            @enhance_trait  // NOTE: this is a recursive call
            trait_def = $trait_def,
            content = { $($trait_content)* },
            fields = {
                $($prev_fields)*
                $(#[$field_attr])*
                let $field_name: $field_type;
            },
            dollar = {$dollar},
        }
    };
    // 4.Parsing trait (finished, trait content doesn't start with a field so rest is the real trait):
    (@enhance_trait
        trait_def = {
            $(#[$attr:meta])*
            $vis:vis trait $trait_name:ident
        },
        content = { $($trait_content:tt)* },
        fields = { $(
            $(#[$field_attr:meta])*
            let $field_name:ident: $field_type:ty;
        )* },
        dollar = {$dollar:tt},
    ) => {
        paste::paste! {
            $(#[$attr])*
            $vis trait $trait_name:
                $crate::TraitEnhance
                + for<'a> $crate::TraitEnhanceType<'a,
                    View = [< $trait_name _View >]<'a>,
                    ViewMut = [< $trait_name _ViewMut >]<'a>
                >
            {
                $($trait_content)*
            }
            #[doc(hidden)]
            #[allow(non_camel_case_types, dead_code)]
            pub struct [< $trait_name _View >]<'a> {
                $($vis $field_name: &'a $field_type,)*
            }
            impl<'a> [< $trait_name _View >]<'a> {
                $vis fn new($($field_name: &'a $field_type),*) -> Self {
                    Self { $($field_name,)* }
                }
            }
            #[doc(hidden)]
            #[allow(non_camel_case_types, dead_code)]
            pub struct [< $trait_name _ViewMut >]<'a> {
                $($vis $field_name: &'a mut $field_type,)*
            }
            impl<'a> [< $trait_name _ViewMut >]<'a> {
                $vis fn new($($field_name: &'a mut $field_type),*) -> Self {
                    Self { $($field_name,)* }
                }
            }
            //  the struct macro part
            #[doc(hidden)]
            #[macro_export] // <-- Only if the trait's visibility is `pub`
            macro_rules! [<$trait_name _for_struct>] { // NOTE: the reexpanded macro is used for rust struct only
                (
                    $dollar (#[$dollar struct_attr:meta])*
                    $dollar vis:vis struct $dollar struct_name:ident {
                        $dollar ( $dollar struct_content:tt )*
                    }
                ) => {
                    $dollar (#[$dollar struct_attr])*
                    $dollar vis struct $dollar struct_name {
                        $dollar ( $dollar  struct_content)*
                        // From outer macro:
                        $(
                            $(#[$field_attr])*
                            $field_name: $field_type,
                        )*
                    }
                    impl<'a> $crate::TraitEnhanceType<'a> for $struct_name {
                        type View = <dyn $trait_name as $crate::TraitEnhanceType<'a>>::View;
                        type ViewMut = <dyn $trait_name as $crate::TraitEnhanceType<'a>>::ViewMut;
                    }
                    impl $crate::TraitEnhance for $struct_name {
                        fn get_fields(&self) -> <Self as $crate::TraitEnhanceType<'_>>::View {
                            <Self as $crate::TraitEnhanceType>::View::new($(
                                &self.$field_name,
                            )*)
                        }
                        fn get_fields_mut(&mut self) -> <Self as $crate::TraitEnhanceType<'_>>::ViewMut {
                            <Self as $crate::TraitEnhanceType>::ViewMut::new($(
                                &mut self.$field_name,
                            )*)
                        }
                    }
                };
            }
        }
    };
}
