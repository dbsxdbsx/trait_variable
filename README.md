# Trait Variable
Making variable fields feasible in trait definition for Rust.

Specifically, this crate enables the use of variable fields in trait definitions through procedural macros along with structs, simulating inheritance found in languages like C++ and Python. This functionality is achieved without modifying Rust's core language features, offering a workaround until a [similar feature](https://github.com/rust-lang/rfcs/pull/1546) might be officially introduced in Rust.

## Usage

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
trait_variable = "*"
```

Then, incorporate the macros into your Rust code as follows:

```rust
/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the VERY TOP of the target trait before any vaild trait item
            x: i32;
        pub y: bool;

        // 2.the order of the original valid trait items doesn't matter
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
    assert_eq!(s.y, true); // s.x is private
}
```
## Technical Details

The above example demonstrates how to define a trait with variable fields and implement it for a struct. The `trait_variable!` macro facilitates the inclusion of variable fields in traits, and the `#[trait_var]` attribute macro automates the implementation details, allowing the struct to use these fields as if they were its own. For a complete example, please refer to the [basic example](https://github.com/dbsxdbsx/trait_variable/blob/main/tests/basic.rs)(There is a more [comprehensive and practical example](https://github.com/dbsxdbsx/trait_variable/blob/main/tests/practical.rs), a more [complex example](https://github.com/dbsxdbsx/trait_variable/blob/main/tests/complex.rs) with generics you can refer to).

Specifically, `x` and `y` are the variable fields of the trait `MyTrait`, and `MyStruct` is a struct that implements `MyTrait`. The `trait_variable!` functional macro will automatically transfer the trait variables into a new parent trait with `_` as the prefix of the trait (`_MyTrait` in this case). Here is an example of how the hidden declarative macro generates a default implementation for `_MyTrait`:
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

In this way, the basics for using trait variables are built while no rule is broken.
And it is easy to guess that any trait function body containing `self.x` as the trait variable identifier would be converted into `self._x()` or `self._x_mut()` accordingly.

Besides, the `#[trait_var(MyTrait)]` attribute macro will first expand the target struct into a form wrapped with a hidden declarative macro (also generated from `trait_variable!`) as stated above in block `way2:` with the macro `MyTrait_for_struct!`, along with an import statement towards to the hidden trait path. Then this hidden declarative macro would further generate a default implementation of `_MyTrait` (not `MyTrait`) for `MyStruct`, which in this case looks like this:

```rust
use crate::PathToMyTrait; // if the trait is defined in another file
impl _MyTrait for MyStruct {
    fn  _x(&self) -> &i32 {
        &self.x
    }
    fn  _x_mut(&self) -> &mut i32 {
        &mut self.x
    }
    fn _y(&self) -> &bool {
        &self.y
    }
    fn _y_mut(&self) -> &mut bool {
        &mut self.y
    }
}
```

Finally, `x` and `y` can be accessed as the struct (trait) fields as if they are the fields of `MyStruct`! with no other field codes! CHEERS~ ^_^; Optionally, the methods defined in trait `MyTrait` could be implemented for `MyStruct` as usual. (NOTE: if the struct is defined in another file other than the file defining the trait, you need to add `use <trait_module>::<hidden_parent_trait_name>;` at the beginning of the file defining the struct -- in this case, the `<hidden_parent_trait_name>` is `_MyTrait`.)

NOTE: In this whole implementation, the `x` and `y` are still not the fields of `MyTrait` but the fields of `MyStruct`, technically. That is: There is no inheritance mechanism at all, since the status/manipulation of `x` and `y` here are only dealt within the implemented struct -- Yes, structs are still defined through interfaces (traits) as usual but not through inheritance.

## Limitations

The code inside the `trait_variable` macro may not receive comprehensive intellisense and lint support from Rust extensions (like [Rust Analyzer](https://github.com/rust-lang/rust-analyzer)). This is because most Rust extensions currently struggle to handle identifiers inside macros effectively. Therefore, when using this crate, you may not have access to full code completion, refactoring, go-to-definition, and other intelligent awareness features. This is a known limitation that may be improved (if it is technically possible) in future versions.

Besides, due to some issue with declarative macro, currently, if the two macros are use  in the sae file, the `trait_variable!` macro should be placed before the `#[trait_var(MyTrait)]` attribute macro, otherwise, the hidden trait `_MyTrait` would not be found. This issue will be fixed in the future.

## Requirements

- Rust edition 2021 or later is required.
- The crate has been tested with Rust version 1.77.1, but the minimum compatible version is not specified.

## Contributing

Contributions are welcome. Feel free to submit pull requests or open issues on the [GitHub repository](https://github.com/dbsxdbsx/trait_variable).

## Todo List

 - [] trait bound(where clause) in struct definition;
 - [] lifetime in struct definition;
 - [] constant generic with array field in struct definition;
 - [] fn `replace_self_field` params necessity check;
 - [] GAT, async methods, etc. in trait definition;
 - [] try to let smart intellisense extension(like `Rust Analyzer`) support idents inside macro(Maybe impossible).

## License

This crate is dual-licensed under either:

- Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (http://opensource.org/licenses/MIT)

Choose the license that best fits your needs.