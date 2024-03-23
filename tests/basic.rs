//! # Basic Usage of the `trait_variable` Macro
//!
//! This file demonstrates the basic usage of the `trait_variable` macro.

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the TOP of the target trait before any function
            x: i32;
        pub y: bool;

        // 2.the order of the function definition doesn't matter
        fn print_x(&self){
            println!("x: `{}`", self.x);
        }
        fn print_y(&self){
            println!("y: `{}`", self.y);
        }
        fn print_all(&self);
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
// way1: use the attribute macro to expand the struct (Recommended)
#[trait_var(MyTrait)]
pub struct MyStruct {
    a: i32,
    pub b: String,
}
// way2: use the hidden declarative macro to expand the struct (Not recommended)
// MyTrait_for_struct! {
//     (_MyTrait) // input the hiddent parent trait
//     pub struct MyStruct { // feel free to add `pub` when needed
//      // feel free to add any fields as usual or leave it empty
//      a: i32,
//      pub b: String,
//     }
// }
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct impl↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
// the struct basic implementation
impl MyStruct {
    pub fn new(a: i32, b: String, x: i32, y: bool) -> Self {
        Self { a, b, x, y }
    }
}
// the trait implementation
impl MyTrait for MyStruct {
    fn print_all(&self) {
        println!("a: `{}`", self.a);
        println!("b: `{}`", self.b);
        self.print_x();
        println!("y: `{}`", self.y);
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct impl↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

#[test]
fn test() {
    let s = MyStruct::new(1, "hello".into(), -2, true);
    s.print_all();
    assert_eq!(s.x, -2); // if not in a unit test, then `self.x`` is not accessible, since it is private
    assert!(s.y);
}
