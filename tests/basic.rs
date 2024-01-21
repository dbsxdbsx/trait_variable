#[cfg(test)]
mod test {
    use trait_variable::{trait_var, trait_variable, TraitEnhance, TraitEnhanceType};

    trait_variable! {
        // the lint is also activated inside the macro, using rust_anaylzer for example
        trait MyTrait {  // feel free to add `pub` when needed
            // 1.put the variable fields definition at the top of the target trait before any function
            let x: i32; // TODO: can't be without variable at present
            let y: bool;

            // 2.the order of the function definition doesn't matter
            fn print_x(&self) {
                println!("x: `{}`", self.get_fields().x);
            }
            fn print_y(&self);
        }
    }

    trait_variable! {
        (MyTrait) // put this at the top of the struct
        struct MyStruct { // feel free to add `pub` when needed
            // feel free to add any fields as usual or leave it empty
            a: i32,
        }
    }

    // #[trait_var(MyTrait)]
    // struct MyStruct {
    //     a: i32,
    // }

    impl MyTrait for MyStruct {
        fn print_y(&self) {
            // println!("{}", self.get_fields().y);
            println!("y: `{}`", self.y);
        }
    }

    #[test]
    fn test_with_attribute_macro() {
        let s = MyStruct {
            a: 2,
            x: 3,
            y: true,
        };
        s.print_x();
        s.print_y();
    }
}
