#[cfg(test)]
mod tests {
    use trait_variable::trait_var;

    #[trait_var2]
    #[test]
    fn test_procedure_macro_syntax() {
        println!("test_module");
    }

    #[test]
    fn another_test_procedure_macro_syntax() {
        println!("another_test_module");
    }
}
