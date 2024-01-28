#[cfg(test)]
mod test {
    use trait_variable::trait_variable;
    use trait_variable_macros::trait_var;

    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    trait_variable! {
        // the lint is also activated inside the macro, using rust_anaylzer for example
        trait MyTrait {  // feel free to add `pub` when needed
            // 1.put the variable fields definition at the top of the target trait before any function
            let x: i32;
            let y: bool;
            let z : String;
            let v : Vec<i32>;

            // 2.the order of the fn definition doesn't matter
            // 2.1.1 fn with default implementation and raw field methods
            fn print_x_raw(&self) {
                println!("x: `{}`", self._x());
            }
            fn change_and_print_x_raw(&mut self, new_num: i32) {
                *self._x_mut() = new_num;
                println!("x: `{}`", self._x());
            }
            fn print_y_raw(&self){
                println!("y: `{}`", self._y());
            }
            fn change_and_print_y_raw(&mut self, new_bool: bool) {
                *self._y_mut() = new_bool;
                println!("y: `{}`", self._y());
            }
            fn print_z_raw(&self){
                println!("z: `{}`", self._z());
            }
            fn change_and_print_z_raw(&mut self, new_str: &str) {
                *self._z_mut() = new_str.into();
                println!("z: `{}`", self._z());
            }
            fn print_v_raw(&self){
                println!("v: `{:?}`", self._v());
            }
            fn change_and_print_v_raw(&mut self, new_vec: Vec<i32>) {
                *self._v_mut() = new_vec;
                println!("v: `{:?}`", self._v());
            }
            // TODO: 2.1.2 fn with default implementation and `self.field`
            fn print_x(&self) {
                let xx = self.x.clone();
                println!("x: `{}`", self.x);
            }
            fn change_and_print_x(&mut self, new_num: i32) {
                self.x = new_num;
                println!("x: `{}`", self.x);
            }
            // 2.2 fn without default implementation
            fn change_and_print_a(&mut self, new_num: i32);
        }
    }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    #[trait_var(MyTrait)]
    struct MyStruct {
        a: i32,
    }
    // the above code would be expanded into this:
    // trait_variable! {
    //     (MyTrait)
    //     struct MyStruct {
    //      a: i32,
    //     }
    // }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓impl block↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    impl MyStruct {
        fn print_a(&self) {
            println!("a: `{}`", self.a);
        }
    }
    impl MyTrait for MyStruct {
        fn change_and_print_a(&mut self, new_num: i32) {
            self.a = new_num;
            println!("a: `{}`", self.a);
        }
        // reload the default implementation with trait variable `self.z`
        fn change_and_print_z_raw(&mut self, new_str: &str) {
            self.z = format!("{}2", new_str);
            println!("(overload method) z: `{}`", self.z);
        }
    }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑impl block↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

    #[test]
    fn test() {
        let mut s = MyStruct {
            a: 2,
            x: 3,
            y: true,
            z: "".to_string(),
            v: vec![1, 2, 3],
        };
        // ----------------------no change value with raw field-methods----------------------
        s.print_x_raw();
        s.print_y_raw();
        s.print_z_raw();
        s.print_v_raw();

        assert_eq!(s.x, 3);
        assert!(s.y);
        assert_eq!(s.z, "");
        assert_eq!(s.v, vec![1, 2, 3]);
        // ----------------------change value with raw field-methods----------------------
        s.change_and_print_x_raw(4);
        s.change_and_print_y_raw(false);
        s.change_and_print_z_raw("hello_world");
        s.change_and_print_v_raw(vec![4, 5, 6]);

        assert_eq!(s.x, 4);
        assert!(!s.y);
        assert_eq!(s.z, "hello_world2");
        assert_eq!(s.v, vec![4, 5, 6]);
        // ----------------------no change value with `self.field`----------------------
        s.print_a();
        // TODO: s.print_x();

        assert_eq!(s.a, 2);
        // ----------------------change value with `self.field`----------------------
        s.change_and_print_a(4);
        // TODO: s.change_and_print_x(4);

        assert_eq!(s.a, 4);
    }
}
