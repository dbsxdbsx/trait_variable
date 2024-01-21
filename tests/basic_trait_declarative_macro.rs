#[cfg(test)]
mod test {
    use trait_variable::{trait_var, trait_variable};

    // trait
    trait_variable! {
        // the lint is also activated inside the macro, using rust_anaylzer for example
        trait MyTrait {  // feel free to add `pub` when needed
            // 1.put the variable fields definition at the top of the target trait before any function
            let x: i32; // TODO: can't be without variable at present
            let y: bool;

            // 2.the order of the function definition doesn't matter
            fn print3(&mut self);
            fn print_x(&self) {
                println!("{}", self.get_fields().x);
            }
            fn print_y(&self) {
                println!("{}", self.get_fields().y);
            }
            fn print2(&self);
        }
    }
    // struct
    #[trait_var(MyTrait)]
    pub struct StructName {
        pub prop: i32,
    }

    #[test]
    fn test_with_attribute_macro() {
        let s = StructName { prop: 4 };
        s.print();
    }
}
