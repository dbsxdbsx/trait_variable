// use trait_variable::trait_var;
// use trait_variable::my_attr_macro;

// /// In this module, the attribute with the argument appears over the enum.
// mod attribute_argument_on_enum {
//     use super::*;

//     trait MyTrait {
//         fn my_function(&self);
//     }

//     #[trait_var(MyTrait)]
//     struct MyStruct;

//     #[test]
//     fn test_trait_var_macro() {
//         let my_struct = MyStruct;
//         my_struct.my_function();
//     }
// }
