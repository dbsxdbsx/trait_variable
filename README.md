# Trait Variable
**(Don't use it. It is still under developement.)**

Make variable fields feasible in trait definition by procedural macros.

This crate provides a procedural macro that allows you to easily implement traits with variable fields just like using inheritance within C++/Python... of parent classes.

And this crate would not be exited if the [fields_in_trait](https://github.com/rust-lang/rfcs/pull/1546) is implemented officially.

## Usage

**(todo: not yet finished)**
Add `trait_variable` to your `Cargo.toml`:

```toml
[dependencies]
trait_variable = "*"
```

Then, use the macro in your Rust code:

```rust
use trait_variable::trait_variable;

#[trait_variable(MyTrait)]
struct MyStruct {
    // Your struct fields
}
```

The `trait_variable` macro will generate a default implementation of `MyTrait` for `MyStruct`, using `todo!()` for all methods not explicitly implemented.

## Requirements

- Rust edition 2021 or later. All code is tested under Rust Version 1.74.0.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue on the [GitHub repository](https://github.com/dbsxdbsx/trait_variable).

## License

This project is licensed under the [MIT License](https://opensource.org/license/mit/).
