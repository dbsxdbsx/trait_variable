mod refine_fn;

#[macro_export]
macro_rules! trait_variable {
    // 1. Entry point after wrapping a trait:
    (
        $(#[$attr:meta])*
        $vis:vis trait $trait_name:ident {
            $($trait_content:tt)*
        }
    ) => {
        $crate::trait_variable!{
            @enhance_trait  // NOTE: go to arm 1.1
            trait_def = {
                $(#[$attr])*
                $vis trait $trait_name
            },
            content = { $($trait_content)* },
            fields = {},
            dollar = {$},
        }
    };
    // 1.1 Parsing trait (has more fields):
    (@enhance_trait
        trait_def = $trait_def:tt,
        content = {
            $(#[$field_attr:meta])*
            let $trait_field_name:ident: $field_type:ty;
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
                let $trait_field_name: $field_type;
            },
            dollar = {$dollar},
        }
    };
    // 1.2 Parsing trait (finished, `content` doesn't start with a field so rest is the real trait):
    (@enhance_trait
        trait_def = {
            $(#[$attr:meta])*
            $vis:vis trait $trait_name:ident
        },
        content = { $($trait_content:tt)* },
        fields = { $(
            $(#[$field_attr:meta])*
            let $trait_field_name:ident: $field_type:ty;
        )* },
        dollar = {$dollar:tt},
    ) => {
        paste::paste! {
            // 1.2.1 the derived parent trait code
            $vis trait [<_ $trait_name>] {
                $(
                    fn [< _$trait_field_name >](&self) -> &$field_type;
                    fn [< _$trait_field_name _mut >](&mut self) -> &mut $field_type;
                )*
            }
            // 1.2.2 the derived basic trait code
            $(#[$attr])*
            #[allow(non_camel_case_types, dead_code)]
            $vis trait $trait_name:
                [<_ $trait_name>] // this is the hidden parent trait
            {
                $crate::refine_fn! {
                    [fns_impls_with_self: ]
                    [fns_impls_with_self_mut: ]
                    [fns_impls_without_self: ]
                    [fns_no_impls: ]
                    $($trait_content)*
                }
            }
            // 1.2.3 the derived macro for struct
            #[doc(hidden)]
            #[macro_export] // <-- Only if the trait's visibility is `pub`
            macro_rules! [<$trait_name _for_struct>] { // NOTE: the reexpanded macro is used for rust struct only
                (
                    $dollar (#[$dollar struct_attr:meta])* // NOTE: make sure the style is consistent with that in arm 2 output
                    $dollar vis:vis struct $dollar struct_name:ident {
                        $dollar ( $dollar struct_content:tt )*
                    }
                ) => {
                    $dollar (#[$dollar struct_attr])*
                    $dollar vis struct $dollar struct_name {
                        $dollar ( $dollar  struct_content)*
                        // NOTE: this part is from root macro:
                        $(
                            $(#[$field_attr])*
                            $trait_field_name: $field_type,
                        )*
                    }
                    impl [<_ $trait_name>] for $struct_name {
                        $(
                            fn [< _$trait_field_name >](&self) -> &$field_type {
                                &self.$trait_field_name
                            }
                            fn [< _$trait_field_name _mut>](&mut self) -> &mut $field_type {
                                &mut self.$trait_field_name
                            }
                        )*
                    }
                };
            }
        }
    };
    // 2. Entry point after wrapping a struct(this arm is invalid if there is no trait wrapped through arm 1):
    (
        ($trait_name:ident) // NOTE: this line is just used as a tag for pattern matching
        // ($trait_name:path) // NOTE: this line is just used as a tag for pattern matching
        // #[trait_tag($trait_name:path)] // this line is just used as a tag
        $(#[$attr:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $trait_field_name:ident : $field_type:ty
            ),* $(,)?
        }
    ) => {
        paste::paste!{
            [<$trait_name _for_struct>] !{ // NOTE: this is the expanded macro from arm 1.2
                $(#[$attr])*
                $vis struct $struct_name {
                    $(
                        $(#[$field_attr:meta])*
                        $field_vis $trait_field_name : $field_type,
                    )*
                }
            }
        }
    };
}
