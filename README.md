# Trait Variable
Make variable fields feasible in trait definition by procedural macros.

This crate provides macros allowing you to easily implement traits with variable fields just like using inheritance within C++/Python... of parent classes. And this crate would not be existed if the [fields_in_trait](https://github.com/rust-lang/rfcs/pull/1546) is implemented officially.

## Usage

Add `trait_variable` to your `Cargo.toml`:

```toml
[dependencies]
trait_variable = "*"
```

Then, use the macros in your Rust code:

```rust
use trait_variable::trait_variable;

#[trait_variable(MyTrait)]
struct MyStruct {
    // Your struct fields
}
```

## TODO list
 - [] add more test cases for various trait field types, like Option, String, Vec, HashMap, etc;
 - [] test with trait with associated types, generic types, async methods, etc;
 - [] omit `use <trait_module>::<_hidden_parent_trait_name>;` statement when using `#[trait_var(<trait_name>)]` for a struct in an extra module;
 - [] try to let smart intellisense extension(like `Rust Analyzer`) support idents inside macro(Maybe impossible).

The `trait_variable` macro will generate a default implementation of `MyTrait` for `MyStruct`, using `todo!()` for all methods not explicitly implemented.

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