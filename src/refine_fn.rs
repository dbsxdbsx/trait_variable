#[macro_export]
macro_rules! refine_fn {
    // TODO: use tt-muncher?
    // 匹配带有函数体的函数
    (
        $(#[$attr:meta])*
        fn $method_name:ident(&self $(, $arg_name:ident : $arg_type:ty)*) -> $ret_type:ty $body:block
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        fn $method_name(&self $(, $arg_name : $arg_type)*) -> $ret_type {
            $crate::refine_fn!(@process_body $body)
        }
        $crate::refine_fn!($($rest)*)
    };
    // 匹配不带函数体的函数
    (
        $(#[$attr:meta])*
        fn $method_name:ident(&self $(, $arg_name:ident : $arg_type:ty)*) -> $ret_type:ty;
        $($rest:tt)*
    ) => {
        $(#[$attr])*
        fn $method_name(&self $(, $arg_name : $arg_type)*) -> $ret_type;
        $crate::refine_fn!($($rest)*)
    };
    // 处理函数体中的替换
    (@process_body {$($body:tt)*}) => {
        {
            $($crate::refine_fn!(@replace_self $body))*
        }
    };
    // 替换`self.field_name`为`self._field_name()`
    (@replace_self self.$field:ident) => {
        self._$field()
    };
    // 对于不需要替换的token，直接返回
    (@replace_self $other:tt) => {
        $other
    };
    // 宏结束条件
    () => {};
}
