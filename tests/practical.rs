//! # Comprehensive and Practical Usage of the `trait_variable` Macro
//!
//! This file demonstrates a more complete and comprehensive usage of the `trait_variable` macro.
//! It is practical, so the trait definition and struct implementation are
//! quite heavy and defined in anthor file.

mod common;
use std::collections::{BTreeMap, HashSet};

use common::MyStruct;

use crate::common::{CustomType, EnumType, MyTrait};

#[test]
fn test() {
    let mut s = MyStruct::new(
        1,
        -2,
        true,
        -3.14,
        vec![-1, 0, 1],
        "hello world",
        Some(0),
        HashSet::from([0, 1, 2]),
        BTreeMap::new(),
        CustomType::new(),
        EnumType::Unit,
    );
    /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test trait fields values↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
    assert_eq!(
        s.test_associated_type_and_constant_value(),
        s.i * MyStruct::VALUE
    );

    s.test_generics_with_trait_display(-1);
    s.test_generics_with_trait_debug(-1);

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
    // test return HashSet<i32>
    assert_eq!(s.test_return_ref_set_i32_by_return_statement(), &s.set_i32);
    assert_eq!(
        s.test_return_mut_ref_set_i32_by_return_statement().clone(),
        s.set_i32
    );
    assert_eq!(s.test_return_ref_set_i32_by_expression(), &s.set_i32);
    assert_eq!(
        s.test_return_mut_ref_set_i32_by_expression().clone(),
        s.set_i32
    );
    assert_eq!(
        s.test_return_cloned_set_i32_by_explicit_clone_return_statement(),
        s.set_i32
    );
    assert_eq!(
        s.test_return_cloned_set_i32_by_explicit_clone_expression(),
        s.set_i32
    );
    // test return BTreeMap<i32, String>
    assert_eq!(
        s.test_return_ref_bmap_by_return_statement(),
        &BTreeMap::new()
    );
    assert_eq!(
        s.test_return_mut_ref_bmap_by_return_statement().clone(),
        BTreeMap::new()
    );
    assert_eq!(s.test_return_ref_bmap_by_expression(), &BTreeMap::new());
    assert_eq!(
        s.test_return_mut_ref_bmap_by_expression().clone(),
        BTreeMap::new()
    );
    assert_eq!(
        s.test_return_cloned_bmap_by_explicit_clone_return_statement(),
        BTreeMap::new()
    );
    assert_eq!(
        s.test_return_cloned_bmap_by_explicit_clone_expression(),
        BTreeMap::new()
    );
    // test return CustomType
    assert_eq!(
        s.test_return_ref_custom_by_return_statement(),
        &CustomType::new()
    );
    assert_eq!(
        s.test_return_mut_ref_custom_by_return_statement(),
        &mut CustomType::new()
    );
    assert_eq!(s.test_return_ref_custom_by_expression(), &CustomType::new());
    assert_eq!(
        s.test_return_mut_ref_custom_by_expression().clone(),
        CustomType::new()
    );
    assert_eq!(
        s.test_return_cloned_custom_by_explicit_clone_return_statement(),
        CustomType::new()
    );
    assert_eq!(
        s.test_return_cloned_custom_by_explicit_clone_expression(),
        CustomType::new()
    );
    // test return EnumType
    assert_eq!(
        s.test_return_ref_enum_by_return_statement(),
        &EnumType::Unit
    );
    assert_eq!(
        s.test_return_mut_ref_enum_by_return_statement(),
        &mut EnumType::Unit
    );
    assert_eq!(s.test_return_ref_enum_by_expression(), &EnumType::Unit);
    assert_eq!(
        s.test_return_mut_ref_enum_by_expression(),
        &mut EnumType::Unit
    );
    assert_eq!(
        s.test_return_cloned_enum_by_explicit_clone_return_statement(),
        EnumType::Unit
    );
    assert_eq!(
        s.test_return_cloned_enum_by_explicit_clone_expression(),
        EnumType::Unit
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
    // test param HashSet<i32>
    s.test_param_set_i32();
    s.test_ref_param_set_i32();
    s.test_mut_ref_param_set_i32();
    // test param BTreeMap<i32, String>
    s.test_param_bmap();
    s.test_ref_param_bmap();
    s.test_mut_ref_param_bmap();
    // test param CustomType
    s.test_param_custom();
    s.test_ref_param_custom();
    s.test_mut_ref_param_custom();
    // test param EnumType
    s.test_param_enum();
    s.test_ref_param_enum();
    s.test_mut_ref_param_enum();
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
    s.test_lambda_for_set_i32();
    s.test_lambda_for_bmap();
    s.test_lambda_for_custom();
    s.test_lambda_for_enum();
    /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test lambda↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
}
