#[cfg(test)]
mod test {
    use trait_variable::trait_variable;
    use trait_variable_macros::trait_var;

    trait_variable! {
        // the lint is also activated inside the macro, using rust_anaylzer for example
        trait MyTrait {  // feel free to add `pub` when needed
            // 1.put the variable fields definition at the top of the target trait before any function
            let x: i32; // TODO: can't be without variable at present
            let y: bool;
            let z : f32;

            // 2.the order of the function definition doesn't matter
            fn print_x(&self);
            // fn print_x(&self) {
                // println!("x: `{}`", self.x);
            // }
            fn print_y(&self);
            fn print_z(&self);

            // fn change_and_print_z(&mut self, new_num: f32) {
            //     *self.get_fields_mut().z = new_num;
            //     println!("z: `{}`", self.get_fields().z);
            // }
        }
    }

    trait_variable! {
        #[trait_var(MyTrait)] // put this at the top of the struct
        struct MyStruct { // feel free to add `pub` when needed
        // feel free to add any fields as usual or leave it empty
         a: i32,
        }
    }
    // TODO: not ok yet
    // #[trait_var(MyTrait)]
    // struct MyStruct {
    //     a: i32,
    // }
    impl MyStruct {
        fn print_a(&self) {
            println!("a: `{}`", self.a);
        }
    }

    impl MyTrait for MyStruct {
        fn print_x(&self) {
            println!("x: `{}`", self.x);
        }
        fn print_y(&self) {
            // println!("{}", self.get_fields().y);
            println!("y: `{}`", self.y);
        }
        fn print_z(&self) {
            println!("z: `{}`", self.z);
        }
    }

    #[test]
    fn test() {
        let mut s = MyStruct {
            a: 2,
            x: 3,
            y: true,
            z: 1.,
        };
        s.print_a();
        s.print_x();
        s.print_y();
        // s.change_and_print_z(3.14);
    }
}
