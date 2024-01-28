#[macro_export]
macro_rules! refine_fn {
    // 1.1 match fns(functions or methods) with default implementation, but with `&self` param prefix
    (
        [fns_impls_with_self: $($fns_impls_with_self:tt)*]
        [fns_impls_with_self_mut: $($fns_impls_with_self_mut:tt)*]
        [fns_impls_without_self: $($fns_impls_without_self:tt)*]
        [fns_no_impls: $($fns_no_impls:tt)*]
        fn $fn_name:ident(& $self:tt, $($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
        $($rest:tt)*
    ) => (
        $crate::refine_fn! {
            [fns_impls_with_self: $($fns_impls_with_self)* /* */ $fn_name ( &$self, $($arg)* ) $($ret_ty)? {$($fn_body)*}]
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
        fn $fn_name:ident(&mut $self:tt, $($arg:tt)*) $(-> $ret_ty:ty)? { $($fn_body:tt)* }
        $($rest:tt)*
    ) => (
        $crate::refine_fn! {
            [fns_impls_with_self: $($fns_impls_with_self)*]
            [fns_impls_with_self_mut: $($fns_impls_with_self_mut)* /* */ $fn_name ( &mut $self, $($arg)* ) $($ret_ty)? {$($fn_body)*}]
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
        [fns_impls_without_self: $($fns_impls_without_self)* /* */ $fn_name ($($arg)*) $($ret_ty)? {$($fn_body)*}]
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
        [fns_impls_with_self: $( $fn_name_impl_with_self:ident (&$self:tt, $($arg_impl_with_self:tt)*) $($ret_ty_impl_with_self:ty)? {$($fn_body_with_self:tt)*} )*]
        [fns_impls_with_self_mut: $( $fn_name_impl_with_self_mut:ident (&mut $self_mut:tt, $($arg_impl_with_self_mut:tt)*) $($ret_ty_impl_with_self_mut:ty)? {$($fn_body_with_self_mut:tt)*} )*]
        [fns_impls_without_self: $( $fn_name_impl_without_self:ident ($($arg_impl_without_self:tt)*) $($ret_ty_impl_without_self:ty)? {$($fn_body_without_self:tt)*} )*]
        [fns_no_impls: $( $fn_name_no_impl:ident ($($arg_no_impl:tt)*) $($ret_ty_no_impl:ty)? ; )*]
    ) => (
        paste::paste!{
            // 2.1.1 copy and refine for each function with default implementation, but with `&self.` prefix
            $(
                fn $fn_name_impl_with_self(&$self, $($arg_impl_with_self)*) $(-> $ret_ty_impl_with_self)? {
                    $crate::refine_fn_body! {
                        $self,  // input `self` here to avoid `self` as a keyword issue.
                        [pre_content: ]
                        $($fn_body_with_self)*
                    }
                    // just for test
                    // $($fn_body)*
                }
            )*
            // 2.1.2 copy and refine for each function with default implementation, but with `&mut self.` prefix
            $(
                fn $fn_name_impl_with_self_mut(&mut $self_mut, $($arg_impl_with_self_mut)*) $(-> $ret_ty_impl_with_self_mut)? {
                    $crate::refine_fn_body! {
                        $self_mut,  // input `self` here to avoid `self` as a keyword issue.
                        [pre_content: ]
                        $($fn_body_with_self_mut)*
                    }
                    // just for test
                    // $($fn_body)*
                }
            )*
            // 2.1.3 copy and refine for each function with default implementation, but without `&(mut)self.` prefix
            $(
                fn $fn_name_impl_without_self($($arg_impl_without_self)*) $(-> $ret_ty_impl_without_self)? {
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
    // 1.1 Match method call with `self.` prefix, match and paste
    (
        $self:tt,
        [pre_content: $($pre_content:tt)*]
        $field:ident.$fn_name:ident ($($args:tt)*) // match fn call with `self.` prefix
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            $self,
            [
                pre_content: $($pre_content)*
                $field.$fn_name($($args:tt)*)  // just copy and paste the original fn code
            ]
            $($rest)*
        }
    };
    // 1.2 Match `self.<field_name>` and replace it
    (
        $self:tt,
        [pre_content: $($pre_content:tt)*]
        $field:ident.$field_name:ident // match field with `self.` prefix
        // self.$field_name:ident // match field with `self.` prefix
        $($rest:tt)*
    ) => {
        $crate::refine_fn_body! {
            $self,
            [
                pre_content: $($pre_content)*
                // todo: check if $field matches $self
                paste::paste! {
                    $self .
                    [<_ $field_name>]
                    ()
                }
                // $crate::match_self_field!($self, $field, $field_name)
            ]
            $($rest)*
        }
    };
    // 2. If no matched pattern, process one token at a time
    (
        $self:tt,
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
        $self:tt,
        [pre_content: $($pre_content:tt)*]
    ) => {
        $($pre_content)*
    };
}

#[macro_export]
// macro_rules! match_self_field {
//     // 当$self和$field相同时，使用这个分支
//     ($self:tt, self, $field_name:ident) => {
//         // 在这里执行当$self和$field相等时的代码
//     };
//     // 当$self和$field不相同时，使用这个分支
//     ($self:tt, $field:ident, $field_name:ident) => {
//         // 在这里执行当$self和$field不相等时的代码
//     };
// }
macro_rules! match_self_field {
    ($self:tt, $field:ident, $field_name:ident) => {
        if $self as *const _ == &$field as *const _ {
            paste::paste! {
                $self .
                [<_ $field_name>]
                ()
            }
        } else {
            paste::paste! {
                $field.$field_name
            }
        }
    };
}
