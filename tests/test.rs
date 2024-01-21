#[cfg(test)]
mod tests {
    use trait_variable::trait_var;

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
                    // parents: Option<Vec<NodeEnum>>, // 父节点列表，有些节点不需要父节点，如“Variable”, 所以用Option
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

    #[trait_var]
    struct StructName {
        pub prop: i32,
    }

    #[test]
    fn another_test_procedure_macro_syntax() {
        let s = StructName { prop: 4 };
        s.print();
    }
}
