mod common;
use common::MyStruct;

use crate::common::MyTrait;

fn test_mutable(num: &mut f32) {
    *num = -1.0;
}

#[test]
fn test() {
    let mut s = MyStruct::new(1, "hello".into(), -2, true, -3.14);
    // 1.test struct fields visibility
    // 1.1 test self contained fields
    // let a = s.a; // Not visible
    let b = s.b.clone();
    // 1.2 test fields generated from trait
    // let x = s.x; // Not visible
    let y = s.y;

    // 2. test methods for the struct fields
    assert_eq!(s.get_print_field_a(), &1);
    assert_eq!(s.get_print_field_b(), &b);
    assert_eq!(s.get_print_field_x(), &-2);
    assert_eq!(s.get_print_field_y(), &y);
    assert_eq!(s.get_print_field_z(), &-3.14);
    assert_eq!(s.change_get_print_field_z(3.14), &3.14);

    // 3. test mutable fields as parameters
    test_mutable(&mut s.z);
    assert_eq!(s.z, -1.0);
    assert_eq!(s.get_print_field_z(), &-1.0);
}
