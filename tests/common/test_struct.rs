use super::{MyTrait, _MyTrait};
use crate::MyTrait_for_struct;
use crate::my_macro;

// way1: use the attribute macro to generate the struct (Recommended)
// use trait_variable_macros::trait_var;
// #[trait_var(MyTrait)]
// struct MyStruct {
//     a: i32,
// }
// way2: use the hidden declarative macro to generate the struct (Not recommended)
MyTrait_for_struct! {
    (_MyTrait) // inputput the hiddent parent trait
    pub struct MyStruct { // feel free to add `pub` when needed
    // feel free to add any fields as usual or leave it empty
    a: i32,
    pub b: String,
    }
}
impl MyStruct {
    pub fn new(a: i32, b: String, x: i32, y: bool, z: f32) -> Self {
        my_macro!();
        Self { a, b, x, y, z }
    }
    pub fn get_print_field_a(&self) -> &i32 {
        println!("a: `{}`", self.a);
        &self.a
    }
    pub fn get_print_field_b(&self) -> &String {
        println!("b: `{}`", self.b);
        &self.b
    }
}
impl MyTrait for MyStruct {
    fn get_print_field_y(&self) -> &bool {
        println!("y: `{}`", self.y);
        &self.y
    }
    fn get_print_field_z(&self) -> &f32 {
        println!("z: `{}`", self.z);
        &self.z
    }
}
