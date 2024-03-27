//! # Complex Usage of the `trait_variable` Macro
//!
//! This file demonstrates the advanced usage of the `trait_variable` macro,
//! focusing on complex generic type parameters and trait bounds.
//! The code in this file is not intended to be practical,
//! but rather to test corner cases and showcase the flexibility of the macro
//! when working with generics.

use core::fmt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use trait_variable::{trait_var, trait_variable};

pub struct CustomGenericType<U> {
    pub name: U,
}

trait ExplicitParentTrait {
    fn get_explicit_parent_trait_name(&self) -> &str {
        "ExplicitParentTrait"
    }
}
trait ExplicitParentTraitWithGeneric<T>
where
    T: AsRef<str> + From<String>,
{
    fn get_explicit_generic_parent_trait_name(&self) -> T {
        T::from("ExplicitParentTraitWithGeneric".to_string())
    }
}

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
trait_variable! {
    pub trait ComplexTrait<P, K: Hash + Eq + Debug + fmt::Display, V>: ExplicitParentTrait + ExplicitParentTraitWithGeneric<P>

    where
        P:AsRef<str> + From<String>,
        V: fmt::Display+ Debug + Clone,
    {
        // the trait variable fields, don't forget to put them at the very TOP place
            id: i32;
        pub data: V;
        pub _phantom_type: K; // TODO: delete?
        pub(crate) cache: HashMap<K, V>;
        custom_generic_obj: CustomGenericType<V>;

        // constant value and associated type
        type HashMapLen;
        type CustomGenericTypeName;
        const STRUCT_ID: i32 ;

        // methods
        fn get_struct_id(&self) -> i32 {
            Self::STRUCT_ID
        }
        fn get_hashmap_len(&self) -> Self::HashMapLen;
        fn get_custom_gereric_type_name(&self) -> &Self::CustomGenericTypeName;
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
// way1: use the attribute macro to expand the struct (Recommended)
#[trait_var(ComplexTrait)]
pub struct ComplexStruct<K, V>;
// way2: use the hidden declarative macro to expand the struct (Not recommended)
// ComplexTrait_for_struct! {
//     pub struct ComplexStruct<K, V> { // feel free to add `pub` when needed
//         // feel free to add any fields as usual or leave it empty
//         pub(crate) extra: String,
//     }
// }
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
impl<K, V> ExplicitParentTrait for ComplexStruct<K, V> {}
impl<K, V> ExplicitParentTraitWithGeneric<String> for ComplexStruct<K, V>
where
    K: Hash + Eq + Debug + fmt::Display,
    V: Debug + Clone + fmt::Display,
{
}

impl<K, V> ComplexTrait<String, K, V> for ComplexStruct<K, V>
where
    K: Hash + Eq + Debug + fmt::Display,
    V: Debug + Clone + fmt::Display,
{
    type HashMapLen = usize;
    type CustomGenericTypeName = V;
    const STRUCT_ID: i32 = 42; // Provide a value for the associated constant

    fn get_hashmap_len(&self) -> Self::HashMapLen {
        self.cache.len()
    }
    fn get_custom_gereric_type_name(&self) -> &Self::CustomGenericTypeName {
        &self.custom_generic_obj.name
    }
}

#[test]
fn test() {
    let mut complex_struct = ComplexStruct {
        id: 42,
        data: "data".to_string(),
        cache: HashMap::new(),
        _phantom_type: 42,
        custom_generic_obj: CustomGenericType {
            name: "hello_world".to_string(),
        },
    };

    complex_struct.cache.insert(42, "key".to_string());

    assert_eq!(
        complex_struct.get_explicit_parent_trait_name(),
        "ExplicitParentTrait"
    );
    assert_eq!(
        complex_struct.get_explicit_generic_parent_trait_name(),
        "ExplicitParentTraitWithGeneric".to_string()
    );
    assert_eq!(complex_struct.get_struct_id(), 42);
    assert_eq!(complex_struct.get_hashmap_len(), 1);
    assert_eq!(complex_struct.get_custom_gereric_type_name(), "hello_world");
}
