#[macro_export]
macro_rules! refine_fn {
    // 1.1 match fns(functions or methods) with default implementation, but with `&self` param prefix
    (
        [fns_impls_with_self: $($fns_impls_with_self:tt)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut:tt)*]
        [fns_impls_without_self: $($fns_impls_without_self:tt)*]
        [fns_no_impls: $($fns_no_impls:tt)*]
        fn $fn_name:ident(&self $($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
        $($rest:tt)*
    ) => (
        $crate::refine_fn! {
            [fns_impls_with_self: $($fns_impls_with_self)* /* */ $fn_name ($($arg)*) $($ret_ty)? {$($fn_body)*}]
            [fns_impls_with_self_mut: $($fns_impls_with_self_mut)*]
            [fns_impls_without_self: $($fns_impls_without_self)*]
            [fns_no_impls: $($fns_no_impls)*]
            $($rest)*
        }
    );
    // 1.2 match fns(functions or methods) with default implementation, but with `&mut self` param prefix
    (
        [fns_impls_with_self: $($fns_impls_with_self:tt)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut:tt)*]
        [fns_impls_without_self: $($fns_impls_without_self:tt)*]
        [fns_no_impls: $($fns_no_impls:tt)*]
        fn $fn_name:ident(&mut self $($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
        $($rest:tt)*
    ) => (
        $crate::refine_fn! {
            [fns_impls_with_self: $($fns_impls_with_self)*]
            [fns_impls_with_self_mut: $($fns_impls_with_self_mut)* /* */ $fn_name ($($arg)*) $($ret_ty)? {$($fn_body)*}]
            [fns_impls_without_self: $($fns_impls_without_self)*]
            [fns_no_impls: $($fns_no_impls)*]
            $($rest)*
        }
    );
     // 1.3 match fns(functions or methods) with default implementation, but without `&(mut)self` param prefix
     (
        [fns_impls_with_self: $($fns_impls_with_self:tt)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut:tt)*]
        [fns_impls_without_self: $($fns_impls_without_self:tt)*]
        [fns_no_impls: $($fns_no_impls:tt)*]
        fn $fn_name:ident($($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
        $($rest:tt)*
    ) => ($crate::refine_fn! {
        [fns_impls_with_self: $($fns_impls_with_self)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut)*]
        [fns_impls_without_self: $($fns_impl)* /* */ $fn_name ($($arg)*) $($ret_ty)? {$($fn_body)*}]
        [fns_no_impls: $($fns_no_impls)*]
        $($rest)*
    });
    // 1.4.match fns(functions or methods) with no default implementation
    (
        [fns_impls_with_self: $($fns_impls_with_self:tt)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut:tt)*]
        [fns_impls_without_self: $($fns_impls_without_self:tt)*]
        [fns_no_impls: $($fns_no_impls:tt)*]
        fn $fn_name_no_impl:ident($($arg_no_impl:tt)*) $(-> $ret_ty_no_impl:ty)? ;
        $($rest:tt)*
    ) => ($crate::refine_fn! {
        [fns_impls_with_self: $($fns_impls_with_self)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut)*]
        [fns_impls_without_self: $($fns_impls_without_self)*]
        [fns_no_impls: $($fns_no_impls)* /* */ $fn_name_no_impl ($($arg_no_impl)*) $($ret_ty_no_impl)?;]
        $($rest)*
    });
    // 2.final output
    (
        [fns_impls_with_self: $( $fn_name_impl_with_self:ident ($($arg_impl_with_self:tt)*) $($ret_ty_impl_with_self:ty)? {$($fn_body_with_self:tt)*} )*]
        [fns_impls_with_self_mut: $( $fn_name_impl_with_self_mut:ident ($($arg_impl_with_self_mut:tt)*) $($ret_ty_impl_with_self_mut:ty)? {$($fn_body_with_self_mut:tt)*} )*]
        [fns_impls_without_self: $( $fn_name_impl_without_self:ident ($($arg_impl_without_self:tt)*) $($ret_ty_impl_without_self:ty)? {$($fn_body_without_self:tt)*} )*]
        [fns_no_impls: $( $fn_name_no_impl:ident ($($arg_no_impl:tt)*) $($ret_ty_no_impl:ty)? ; )*]
    ) => (
        paste::paste!{
            // 2.1.1 copy and refine for each function with default implementation, but with `&self.` prefix
            $(
                fn $fn_name_impl_with_self(& self $($arg_impl_with_self)*) $(-> $ret_ty_impl_with_self)? {
                    $crate::refine_fn_body! {
                        self,
                        [pre_content: ]
                        $($fn_body_with_self)*
                    }
                    // just for test
                    // $($fn_body)*
                }
            )*
            // 2.1.2 copy and refine for each function with default implementation, but with `&mut self.` prefix
            $(
                fn $fn_name_impl_with_self_mut(&mut self $($arg_impl_with_self_mut)*) $(-> $ret_ty_impl_with_self_mut)? {
                    $crate::refine_fn_body! {
                        self,
                        [pre_content: ]
                        $($fn_body_with_self_mut)*
                    }
                    // just for test
                    // $($fn_body)*
                }
            )*
            // 2.1.3 copy and refine for each function with default implementation, but without `&(mut)self.` prefix
            $(
                fn $fns_impls_without_self($($arg_impl_without_self)*) $(-> $ret_ty_impl_without_self)? {
                    $($fn_body_without_self)*
                }
            )*
            // 2.1.4 copy and paste for each function with no default implementation
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
    // // 0.
    // (@work_around_self
    //     $($self:tt)* | $fn_name:ident | $($args:tt)*
    // ) => {
    //     $self$fn_name($($args)*)
    // };
    // 1.1 Match method call with `self.` prefix, match and paste
    (
        $self:ident,
        [pre_content: $($pre_content:tt)*]
        self.$fn_name:ident ($($args:tt)*) // match fn call with `self.` prefix
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            $self,
            [
                pre_content: $($pre_content)*
                // just copy and paste the original fn code
                $self.$fn_name($($args:tt)*)
            ]
            $($rest)*
        }
    };
    // 1.2 Match `self.<field_name>` and replace it
    (
        $self:ident,
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
        $self:ident,
        [pre_content: $($pre_content:tt)*]
        $token:tt
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            $self,
            [pre_content: $($pre_content)* $token]
            $($rest)*
        }
    };
    // 3. Base case: no more tokens to process
    (
        $self:ident,
        [pre_content: $($pre_content:tt)*]
    ) => {
        $($pre_content)*
    };
}
