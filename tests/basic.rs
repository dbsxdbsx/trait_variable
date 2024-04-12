//! # Basic Usage of the `trait_variable` Macro
//!
//! This file demonstrates the basic usage of the `trait_variable` macro.

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait BasicTrait {  // feel free to add `pub` when needed
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
#[trait_var(BasicTrait)]
struct MyStruct {
    // feel free to add `pub` when needed
    // feel free to add any fields as usual or leave it empty
    a: i32,
    pub b: String,
}
// way2: use the hidden declarative macro to expand the struct (Not recommended)
// BasicTrait_for_struct! {
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
impl BasicTrait for MyStruct {
    fn print_all(&self) {
        println!("a: `{}`", self.a);
        println!("b: `{}`", self.b);
        self.print_x();
        println!("y: `{}`", self.y);
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct impl↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓anthor struct for test↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
#[trait_var(BasicTrait)]
struct AnthorStruct {
    c: i32,
    d: bool,
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑anthor struct for test↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

#[test]
fn test() {
    // create a struct instance using the trait_variable macro
    let s = MyStruct::new(1, "hello".into(), -2, true);
    s.print_all();
    assert_eq!(s.a, 1);
    assert_eq!(s.b, "hello");
    assert_eq!(s.x, -2); // if not in a unit test, then `self.x`` is not accessible, since it is private
    assert!(s.y);

    // create another struct instance using the trait_variable macro
    let s2 = AnthorStruct {
        c: 42,
        d: true,
        x: -2,
        y: true,
    };
    assert_eq!(s2.c, 42);
    assert!(s2.d);
    assert_eq!(s2.x, -2);
    assert!(s2.y);
}
