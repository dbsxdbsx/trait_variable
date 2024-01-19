// // tests/trait_var_macro.rs
// use trait_variable::trait_var;

// trait MyTrait {
//     fn my_function(&self);
// }

// #[trait_var(MyTrait)]
// struct MyStruct;

// // impl MyTrait for MyStruct {
// //     fn my_function(&self) {
// //         println!("MyTrait has been implemented for MyStruct.");
// //     }
// // }

// #[test]
// fn test_trait_var_macro() {
//     let my_struct = MyStruct;
//     my_struct.my_function();
// }
