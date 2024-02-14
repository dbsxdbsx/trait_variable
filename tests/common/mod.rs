//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the top of the target trait before any function
            x: i32;
        pub y: bool;
        pub z: f32;

        // 2.the order of the function definition doesn't matter
        fn get_print_field_x(&self) -> &i32{
            // println!("x: `{}`", self._x());// ok
            println!("x: `{}`", self.x);// TODO: make self.<> valid
            // self._x() // ok
            self.x
        }
        fn get_print_field_y(&self) -> &bool;
        fn get_print_field_z(&self) -> &f32;
        fn change_get_print_field_z(&mut self, func: fn(&mut f32))->&f32 {
            // modify the field by assignment operation
            // (*self._z_mut()) = new_num; // ok
            // &mut self.z = new_num; // TODO

            // modify the field by function call
            func(&mut self._z_mut()); // ok
            // self.test();
            // self.test(&mut self.z);
            println!("z: `{}`",self.z);

            self.z
        }

        fn get_cloned_trait_field(&self) -> (i32, bool, f32) {
            (*self.x, *self.y, *self.z)
        }
    }
}
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
// way1: use the attribute macro to generate the struct (Recommended)
#[trait_var(MyTrait)]
pub struct MyStruct {
    a: i32,
    pub b: String,
}
// way2: use the hidden declarative macro to generate the struct (Not recommended)
// MyTrait_for_struct! {
//     (_MyTrait) // inputput the hiddent parent trait
//     pub struct MyStruct { // TODO: feel free to add `pub` when needed
//      // feel free to add any fields as usual or leave it empty
//      a: i32,
//      pub b: String,
//     }
// }

impl MyStruct {
    pub fn new(a: i32, b: String, x: i32, y: bool, z: f32) -> Self {
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
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
// TODO: delete the following code with file `test_struct.rs`
// use trait_variable_macros::trait_variable;
// mod test_struct;
// pub use test_struct::MyStruct;

// trait_variable! {
//     pub trait MyTrait {  // feel free to add `pub` when needed
//         // 1.put the variable fields definition at the top of the target trait before any function
//             x: i32;
//         pub y: bool;
//         pub z: f32;

//         // 2.the order of the function definition doesn't matter
//         fn get_print_field_x(&self) -> &i32{
//             println!("x: `{}`", self._x());// TODO: make self.<> valid
//             self._x()
//             // &self.x
//         }
//         fn get_print_field_y(&self) -> &bool;
//         fn get_print_field_z(&self) -> &f32;
//         fn change_and_print_z(&mut self, _new_num: f32) {
//             // self.z = new_num; // TODO
//             // println!("z: `{}`",self.z);
//         }
//     }
// }

// #[macro_export]
// macro_rules! my_macro {
//     () => {
//         println!("Hello from macro in module a");
//     };
// }
