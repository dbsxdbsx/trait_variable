//! # Comprehensive Usage of the `trait_variable` Macro
//!
//! This file demonstrates a more complete and comprehensive usage of the `trait_variable` macro.
mod common;
use common::MyStruct;

use crate::common::MyTrait;

#[test]
fn test() {
    let mut s = MyStruct::new(1, -2, true, -3.14, vec![-1, 0, 1]);
    // here only test advanced methods, no need to test the basics as in module `basic.rs`
    s.test_macro();
    s.test_assigntment();
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
    // test return vec
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
    //
    s.test_param_i32();
    s.test_ref_param_i32();
    s.test_mut_ref_param_i32();
    //
    s.test_param_vec_i32();
    s.test_ref_param_vec_i32();
    s.test_mut_ref_param_vec_i32();
    //
    s.test_if_else();
    s.test_match_arm();
    s.test_raw_loop();
    s.test_for_loop();
    //
    s.test_lambda_for_i32();
}
