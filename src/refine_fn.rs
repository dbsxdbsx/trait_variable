#[macro_export]
macro_rules! refine_fn {
    // 1.match fns(functions or methods) with default implementation
    (
        [fns_impls: $($fns_impl:tt)*]
        [fns_no_impls: $($fns_no_impl:tt)*]
        fn $fn_name:ident($($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
        $($rest:tt)*
    ) => ($crate::refine_fn! {
        [fns_impls: $($fns_impl)* /* */ $fn_name ($($arg)*) $($ret_ty)? {$($fn_body)*}]
        [fns_no_impls: $($fns_no_impl)*]
        $($rest)*
    });
    // 2.match fns(functions or methods) with no default implementation
    (
        [fns_impls: $($fns_impl:tt)*]
        [fns_no_impls: $($fns_no_impl:tt)*]
        fn $fn_name_no_impl:ident($($arg_no_impl:tt)*) $(-> $ret_ty_no_impl:ty)? ;
        $($rest:tt)*
    ) => ($crate::refine_fn! {
        [fns_impls: $($fns_impl)*]
        [fns_no_impls: $($fns_no_impl)* /* */ $fn_name_no_impl ($($arg_no_impl)*) $($ret_ty_no_impl)?;]
        $($rest)*
    });
    // 3.final output
    (
        [fns_impls: $( $fn_name_impl:ident ($($arg_impl:tt)*) $($ret_ty_impl:ty)? {$($fn_body:tt)*} )*]
        [fns_no_impls: $( $fn_name_no_impl:ident ($($arg_no_impl:tt)*) $($ret_ty_no_impl:ty)? ; )*]
    ) => (
        paste::paste!{
            // 3.1 copy and paste for each function with default implementation
            $(
                fn $fn_name_impl($($arg_impl)*) $(-> $ret_ty_impl)? {
                    // TODO:
                    // $crate::refine_fn_body! {
                    //     [pre_content: ]
                    //     $($fn_body)*
                    // }

                    //
                    $($fn_body)*
                }
            )*
            // 3.2 copy and paste for each function with no default implementation
            $(
                fn $fn_name_no_impl($($arg_no_impl)*) $(-> $ret_ty_no_impl)? ;
            )*
        }
    );
}

// --------------------------------------------
#[macro_export]
/// refine the body content of a function/method, excluding braces
/// replace all `self.<field_name>` to `self._<field_name>()`
macro_rules! refine_fn_body {
    // 1.1 Match method call with `self.` prefix, match and paste
    (
        [pre_content: $($pre_content:tt)*]
        self.$fn_name:ident ($($args:tt)*)
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            [
                pre_content: $($pre_content)*
                // just copy and paste the original fn code
                self.$fn_name($($args:tt)*)
            ]
            $($rest)*
        }
    };
    // 1.2 Match `self.<field_name>` and replace it
    (
        [pre_content: $($pre_content:tt)*]
        self.$field_name:ident
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            [
                pre_content: $($pre_content)*
                // todo:
                paste::paste!{
                    self.
                    [<_ $field_name>]
                    ()
                }
            ]
            $($rest)*
        }
    };
    // 2. If no matched pattern, process one token at a time
    (
        [pre_content: $($pre_content:tt)*]
        $token:tt
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            [pre_content: $($pre_content)* $token] $($rest)*
        }
    };
    // 3. Base case: no more tokens to process
    ([pre_content: $($pre_content:tt)*]) => {
        $($pre_content)*
    };
}
