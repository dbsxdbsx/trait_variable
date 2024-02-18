//! # Complicated Usage of the `trait_variable` Macro
//!
//! This file demonstrates a more complete and complicated usage of the `trait_variable` macro.
mod common;
use common::MyStruct;

use crate::common::MyTrait;

#[test]
fn test() {
    let mut s = MyStruct::new(1, "hello".into(), -2, true, -3.14);
    // 1, print
    s.print_all_trait_fields();
    // 2.test struct fields visibility
    // 2.1 test self contained fields
    // let a = s.a; // Not visible
    let b = s.b.clone();
    // 2.2 test fields generated from trait
    // let x = s.x; // Not visible
    let y = s.y;

    // 3. test methods for the struct fields
    assert_eq!(s.get_print_field_a(), &1);
    assert_eq!(s.get_print_field_b(), &b);
    assert_eq!(s.get_print_field_x(), &-2);
    assert_eq!(s.get_print_field_y(), &y);
    assert_eq!(s.get_print_field_z(), &-3.14);
    assert_eq!(
        s.change_get_print_field_z(
            |z| {},
            |z| {
                *z = -*z;
            }
        ),
        &-3.14
    );
    assert_eq!(s.get_cloned_trait_field(), (-2, s.y, -3.14));
}
