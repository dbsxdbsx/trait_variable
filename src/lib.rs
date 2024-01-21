#[macro_export]
macro_rules! add_print {
    (pub struct $struct_name:ident {
        $($body:tt)*
    }) => {
        add_print!(@impl pub struct $struct_name { $($body)* });
    };
    (struct $struct_name:ident {
        $($body:tt)*
    }) => {
        add_print!(@impl struct $struct_name { $($body)* });
    };
    //
    (@impl $vis:vis struct $struct_name:ident {
        $($user_field_vis:vis $user_field_name:ident : $user_field_type:ty),*
        $(,)?
    }) => {
        paste::paste! {
            $vis struct $struct_name {
                customized_field: i32,
                // 以下是自定义的字段
                $($user_field_vis $user_field_name : $user_field_type,)*
            }

            impl [<$struct_name>] {
                pub fn print(&self) {
                    println!("Hello, xxxworld!");
                }
            }
        }
    }
}

pub use trait_variable_macros::trait_var;
