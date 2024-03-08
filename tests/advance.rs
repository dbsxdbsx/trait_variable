//! # Comprehensive Usage of the `trait_variable` Macro
//!
//! This file demonstrates a more complete and comprehensive usage of the `trait_variable` macro.
mod common;
use common::MyStruct;

use crate::common::MyTrait;

#[test]
fn test() {
    let mut s = MyStruct::new(1, -2, true, -3.14, vec![-1, 0, 1], "hello world", Some(0));
    // here only test advanced methods, no need to test the basics as in module `basic.rs`
    s.test_macro();
    s.test_assigntment();

    /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test return type↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
    // test return i32
    assert_eq!(s.test_return_ref_i32_by_return_statement(), &-2);
    assert_eq!(s.test_return_mut_ref_i32_by_return_statement(), &-2);
    assert_eq!(s.test_return_ref_i32_by_expression(), &-2);
    assert_eq!(s.test_return_mut_ref_i32_by_expression(), &-2);
    assert_eq!(
        s.test_return_cloned_i32_by_explicit_clone_return_statement(),
        -2
    );
    assert_eq!(
        s.test_return_cloned_i32_by_implicit_clone_return_statement(),
        -2
    );
    assert_eq!(s.test_return_cloned_i32_by_explicit_clone_expression(), -2);
    assert_eq!(s.test_return_cloned_i32_by_implicit_clone_expression(), -2);
    // test return Vec<i32>
    assert_eq!(s.test_return_ref_vec_by_return_statement(), vec![-1, 0, 1]);
    assert_eq!(
        s.test_return_mut_ref_vec_by_return_statement(),
        vec![-1, 0, 1]
    );
    assert_eq!(s.test_return_ref_vec_by_expression(), vec![-1, 0, 1]);
    assert_eq!(s.test_return_mut_ref_vec_by_expression(), vec![-1, 0, 1]);
    assert_eq!(
        s.test_return_cloned_vec_by_explicit_clone_return_statement(),
        vec![-1, 0, 1]
    );
    assert_eq!(
        s.test_return_cloned_vec_by_explicit_clone_expression(),
        vec![-1, 0, 1]
    );
    // test return String, &str
    assert_eq!(s.test_return_ref_str_by_return_statement(), "hello world");
    assert_eq!(
        s.test_return_mut_ref_string_by_return_statement(),
        "hello world"
    );
    assert_eq!(s.test_return_ref_string_by_expression(), "hello world");
    assert_eq!(s.test_return_mut_ref_string_by_expression(), "hello world");
    assert_eq!(
        s.test_return_cloned_string_by_explicit_clone_return_statement(),
        "hello world"
    );
    assert_eq!(
        s.test_return_cloned_string_by_explicit_clone_expression(),
        "hello world"
    );
    assert_eq!(s.test_return_ref_str_by_return_statement(), "hello world");
    assert_eq!(s.test_return_ref_str_by_expression(), "hello world");
    // test return Option<i32>
    assert_eq!(s.test_return_ref_opt_i32_by_return_statement(), &Some(0));
    assert_eq!(
        s.test_return_mut_ref_opt_i32_by_return_statement(),
        &mut Some(0)
    );
    assert_eq!(s.test_return_ref_opt_i32_by_expression(), &Some(0));
    assert_eq!(s.test_return_mut_ref_opt_i32_by_expression(), &mut Some(0));
    assert_eq!(
        s.test_return_cloned_opt_i32_by_explicit_clone_return_statement(),
        Some(0)
    );
    assert_eq!(
        s.test_return_cloned_opt_i32_by_implicit_clone_return_statement(),
        Some(0)
    );
    assert_eq!(
        s.test_return_cloned_opt_i32_by_explicit_clone_expression(),
        Some(0)
    );
    assert_eq!(
        s.test_return_cloned_opt_i32_by_implicit_clone_expression(),
        Some(0)
    );
    // test return tuple (i32, String, Vec<i32>)
    assert_eq!(
        s.test_return_ref_tuple_by_return_statement(),
        &(0, "".into(), vec![])
    );
    assert_eq!(
        s.test_return_mut_ref_tuple_by_return_statement(),
        &mut (0, "".into(), vec![])
    );
    assert_eq!(
        s.test_return_ref_tuple_by_expression(),
        &(0, "".into(), vec![])
    );
    assert_eq!(
        s.test_return_mut_ref_tuple_by_expression(),
        &mut (0, "".into(), vec![])
    );
    assert_eq!(
        s.test_return_cloned_tuple_by_explicit_clone_return_statement(),
        (0, "".into(), vec![])
    );
    assert_eq!(
        s.test_return_cloned_tuple_by_explicit_clone_expression(),
        (0, "".into(), vec![])
    );
    /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test return type↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

    /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test param↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
    // test param i32
    s.test_param_i32();
    s.test_ref_param_i32();
    s.test_mut_ref_param_i32();
    // test param vec i32
    s.test_param_vec_i32();
    s.test_ref_param_vec_i32();
    s.test_mut_ref_param_vec_i32();
    // test param String, &str
    s.test_param_string();
    s.test_ref_param_str();
    s.test_mut_ref_param_str();
    // test param Option<i32>
    s.test_param_opt_i32();
    s.test_ref_param_opt_i32();
    s.test_mut_ref_param_opt_i32();
    // test param tuple (i32, String, Vec<i32>)
    s.test_param_tuple();
    s.test_ref_param_tuple();
    s.test_mut_ref_param_tuple();
    /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test param↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

    /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓conditional/loop↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
    s.test_if_else();
    s.test_match_arm();
    s.test_raw_loop();
    s.test_for_loop();
    /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑conditional/loop↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

    /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test lambda↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
    s.test_lambda_for_i32();
    s.test_lambda_for_vec_i32();
    s.test_lambda_for_string_and_str();
    s.test_lambda_for_opt_i32();
    s.test_lambda_for_tuple();
    /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test lambda↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
}
