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
                    $crate::refine_fn_body! { $($fn_body)* }
                    // $($fn_body)* // TODO: do more with the `fn_body`
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
macro_rules! refine_fn_body {
    // TODO: use tt-muncher to rewrite the whole!!!!!!!!!!
    // 匹配 self.<any_name> 并替换为 self._<any_name>()
    ($($body:tt)*) => {
        $($body)*
    };
}

// 定义宏`replace_expr`来执行实际的替换操作
#[macro_export]
macro_rules! replace_expr {
    // 匹配 self.<any_name> 并替换为 self._<any_name>()
    (self.$name:ident) => {
        self._$name()
    };
    // 递归地处理每个token tree
    ($first:tt $($rest:tt)*) => {
        replace_expr!($first)
        replace_expr!($($rest)*)
    };
    // 匹配任何其他单个token tree并保持不变
    ($other:tt) => {
        $other
    };
}
