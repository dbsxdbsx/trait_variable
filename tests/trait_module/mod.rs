use trait_variable::trait_variable;

trait_variable! {
    pub trait MyTraitForSeparateDef {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the top of the target trait before any function
        pub x: i32;
            y: bool;
        // 2. fn without default implementation
        fn change_and_print_x(&mut self, new_num: i32);
    }
}
