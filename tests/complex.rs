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

pub struct CustomGenericType<T, U> {
    pub name: U,
    pub nick_name: T,
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
        P: fmt::Display + AsRef<str> + From<String>,
        V: fmt::Display + Debug + Clone,
    {
        // the trait variable fields, don't forget to put them at the very TOP place
        pub data: V;
            id: i32;
        pub(crate) cache: HashMap<K, V>;
        custom_generic_obj: CustomGenericType<P, V>;

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
        fn get_custom_gereric_type_nick_name(&self) -> &P;
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
// NOTE: when using generics with the macro, please use exactly the SAME generic tags as the trait does;
// Besides, TODO:
// way1: use the attribute macro to expand the struct (Recommended)
#[trait_var(ComplexTrait)]
pub struct ComplexStruct<I, P, K, V>
// TODO: test the struct where clause
//     where
//     I: Copy + std::ops::Add<Output = I>,
{
    i: I,
    s: P, // NOTE: the type `P` is also used as a generic type for the trait varialbe field and explicit parent trait
}
// way2: use the hidden declarative macro to expand the struct (Not recommended)
// ComplexTrait_for_struct! {
//     struct ComplexStruct<I, P, K, V> // feel free to add `pub` when needed
// TODO: test the struct where clause
//     where
//     I: Copy + std::ops::Add<Output = I>,
//     {
//        i: I,
//        s: P,
//     }
// }
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
impl<I, P, K, V> ComplexStruct<I, P, K, V>
where
    I: Copy + std::ops::Add<Output = I>,
    P: fmt::Display,
{
    fn add_i(&self, num: I) -> I {
        self.i + num
    }
    fn print_and_get_s(&self) -> &P {
        println!("s: `{}`", self.s);
        &self.s
    }
}

impl<I, P, K, V> ExplicitParentTrait for ComplexStruct<I, P, K, V> {}
impl<I, P, K, V> ExplicitParentTraitWithGeneric<P> for ComplexStruct<I, P, K, V>
// NOTE: It is NOT yet supported for specific generic type usage like this:
// `impl<I, P, K, V> ExplicitParentTraitWithGeneric<String> for ComplexStruct<I, P, K, V>`,
// because in this intricate example, the type `P` is not only used as a generic type for a parent trait,
// but also used as a generic type for the trait varialbe field, which means that the type `P`
// need to be specifically handled for the hidden trait `_ComplexTrait`.
// While if the `P` is NOT mixly used both for the trait varialbe field and an explicit parent trait,
// then it is supported.
where
    P: AsRef<str> + From<String>
{
}

impl<I, P, K, V> ComplexTrait<P, K, V> for ComplexStruct<I, P, K, V>
// NOTE: In this intricate example, for specific generic type usage like this:
// `impl<I, P, K, V> ComplexTrait<String, K, V> for ComplexStruct<I, P, K, V>`,
// the reason is the same as above.
where
    P: fmt::Display + AsRef<str> + From<String>,
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
    fn get_custom_gereric_type_nick_name(&self) -> &P {
        &self.custom_generic_obj.nick_name
    }
}

#[test]
fn test() {
    let mut complex_struct = ComplexStruct {
        i: 1,
        id: 42,
        s: "s".to_string(),
        data: "data".to_string(),
        cache: HashMap::new(),
        custom_generic_obj: CustomGenericType {
            name: "name".to_string(),
            nick_name: "nick name".to_string(),
        },
    };

    assert_eq!(complex_struct.add_i(2), 3);
    assert_eq!(complex_struct.print_and_get_s(), "s");
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
    assert_eq!(complex_struct.get_custom_gereric_type_name(), "name");
    assert_eq!(
        complex_struct.get_custom_gereric_type_nick_name(),
        "nick name"
    );
}
