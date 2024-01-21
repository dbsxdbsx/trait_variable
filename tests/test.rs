#[cfg(test)]
mod tests {
    use trait_variable::trait_var;

    #[trait_var]
    struct StructName {
        pub prop: i32,
    }

    #[test]
    fn another_test_procedure_macro_syntax() {
        let s = StructName { prop: 4 };
        s.print();
    }
}
