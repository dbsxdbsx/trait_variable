# Trait Variable
Make variable fields feasible in trait definition by procedural macros.

This crate provides macros allowing you to easily implement traits with variable fields just like using inheritance within C++/Python... of parent classes. And this crate would not be existed if the [fields_in_trait](https://github.com/rust-lang/rfcs/pull/1546) is implemented officially.

## Usage

Add `trait_variable` to your `Cargo.toml`:

```toml
[dependencies]
trait_variable = "*"
```

Then, use the macros in your Rust code like this:

```rust
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
// way1: use the attribute macro to generate the struct (Recommended)
#[trait_var(MyTrait)]
pub struct MyStruct {
    a: i32,
    pub b: String,
}
// way2: use the hidden declarative macro to generate the struct (Not recommended)
// MyTrait_for_struct! {
//     (_MyTrait) // inputput the hiddent parent trait
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
        println!("x: `{}`", self.x);
        println!("y: `{}`", self.y);
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct impl↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

#[test]
fn test() {
    let s = MyStruct::new(1, "hello".into(), -2, true);
    s.print_all();
    assert_eq!(s.y, true);
}
```
Here `x` and `y` are the variable fields of the trait `MyTrait`, and `MyStruct` is a struct that implements `MyTrait`. The `trait_variable!` functional macro will automatically transfer the trait variables into a new parent trait with `_` as prefix of the trait(`_MyTrait` in this case) like this:
```rust
pub(crate) trait _MyTrait {
    fn _x(&self) -> &i32;
    fn _x_mut(&self) -> &mut i32;
    fn _y(&self) -> &bool;
    fn _y_mut(&self) -> &mut bool;
}
pub(crate) trait MyTrait: _MyTrait {
  // 1. the trait variable is erased here
  // 2. the original trait definition is kept here
  // ...
}
```
In this way, the basics for using trait variables are built while no rule broken.
And it is easy to guess for any function body contains `self.x` the trait variable like ident, these ident would be converted into `self._x()` or `self._x_mut()` accordingly.

Besides, the `#[trait_var(MyTrait)]` attribute macro will first expand the target struct into some form wrapped with a hidden declarative macro as stated above in block `way2:`. Then the this hidden declarative macro would furthur generate a default implementation of `_MyTrait`(Not `MyTrait`) for `MyStruct`, which in this case is like this:
```rust
impl _MyTrait for MyStruct {
    fn  _x(&self) -> &i32 {
        self.x
    }
    fn  _x_mut(&self) -> &mut i32 {
        self.x
    }
    fn _y(&self) -> &bool {
        self.y
    }
    fn _y_mut(&self) -> &mut bool {
        self.y
    }
}
```
Finally, `x` and `y` can be accessed as the struct (trait) fields as if they are the fields of `MyStruct`! with no other field codes! CHEERS~ ^_^; Optionally, the methods defined in trait `MyTrait` could be implemented for `MyStruct` as usual.(NOTE: if the struct is defined in another file other than the file defining the trait, you need to add `use <trait_module>::<hidden_parent_trait_name>;` at the beginning of the file defining the struct -- in this case the `<hidden_parent_trait_name>` is `_MyTrait`.)

NOTE: In this whole implementation, the `x` and `y` are still not the fields of `MyTrait` but the fields of `MyStruct, techniqically. That is: There is no inheritance mechanism at all, since the status/manipulation of `x` and `y` here are only dealt within the implemented struct-- Yes, structs are defined through interface(traits) as usual but not through inheritance.


## TODO list
 - [] replace_self_field重构，返回类型是&mut的情况 ,if block表达式（复杂模式）,  &mut self.z 参数测试;
  to English comment;
 - [] add more test cases for various trait field types, like Option, String, Vec, HashMap, etc;
 - [] test with trait with constant field, associated types, generic types, async methods, etc;
 - [] test left value with tuple, like `(<trait_field>, _)`;
 - [] omit `use <trait_module>::<_hidden_parent_trait_name>;` statement when using `#[trait_var(<trait_name>)]` for a struct in an extra module;
 - [] try to let smart intellisense extension(like `Rust Analyzer`) support idents inside macro(Maybe impossible).

## Requirements

- Rust edition 2021 or later. All code is tested under Rust Version 1.76.0. Not Sure the minimum version.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue on the [GitHub repository](https://github.com/dbsxdbsx/trait_variable).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.