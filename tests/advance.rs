//! # Complicated Usage of the `trait_variable` Macro
//!
//! This file demonstrates a more complete and complicated usage of the `trait_variable` macro.
mod common;
use common::MyStruct;

use crate::common::MyTrait;

#[test]
fn test() {
    let mut s = MyStruct::new(1, -2, true, -3.14);

    s.test_macro();
    s.test_assigntment();
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
    s.test_param_i32();
    s.test_ref_param_i32();
    s.test_mut_ref_param_i32();
    // // 3. test methods for the struct fields
    // assert_eq!(s.get_print_field_a(), &1);
    // assert_eq!(s.get_print_field_b(), &s.b);
    // assert_eq!(s.test_return_ref_x_by_return_statement(), &-2);
    // assert_eq!(s.get_print_field_y(), &s.y);
    // assert_eq!(s.get_print_field_z(), &-3.14);
    // assert_eq!(
    //     s.change_get_print_field_z(
    //         |z| {},
    //         |z| {
    //             *z = -*z;
    //         }
    //     ),
    //     &-3.14
    // );
    // assert_eq!(s.get_cloned_trait_field(), (-2, s.y, -3.14));
}
