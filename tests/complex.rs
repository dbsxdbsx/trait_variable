use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use trait_variable::{trait_var, trait_variable};

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
trait_variable! {
    pub trait ComplexTrait<U>
    // TODO: pub trait ComplexTrait<T, U>: // TODO: ParentTrait
    where
        // T: Debug + Clone, // TODO:
        U: Hash + Eq,
    {
        // the trait variable fields, don't forget to put them at the very TOP place
        pub id: i32;
        pub data: U;
        // pub(crate) cache: HashMap<U, T>; // TODO:

        // constant value and associated type
        type Output;
        const THRESHOLD: i32 ;

        // method that uses associated type
        fn process_data(&self) -> Self::Output;
    }
}

/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
// way1: use the attribute macro to expand the struct (Recommended)
// #[trait_var(ComplexTrait)]
// pub struct ComplexStruct<T, U> {
//     pub(crate) extra: String,
// }

// way2: use the hidden declarative macro to expand the struct (Not recommended)
ComplexTrait_for_struct! {
    // TODO: pub struct ComplexStruct<T, U> { // feel free to add `pub` when needed
    pub struct ComplexStruct<U> { // feel free to add `pub` when needed
     // feel free to add any fields as usual or leave it empty
     pub(crate) extra: String,
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
// // 为 ComplexStruct 实现 ComplexTrait
// impl<T, U> ComplexTrait<T, U> for ComplexStruct<T, U>
// where
//     T: Debug + Clone,
//     U: Hash + Eq,
// {
//     type Output = usize;

//     fn process_data(&self) -> Self::Output {
//         // 处理数据并返回结果
//         self.cache.len()
//     }
// }

// // TODO: delete?
// // // 实现父特性
// // impl<T, U> ParentTrait for ComplexStruct<T, U> {
// //     fn parent_method(&self) {
// //         println!("ComplexStruct parent method");
// //     }
// // }

// // fn main() {
// //     // 创建 ComplexStruct 实例
// //     let mut complex_struct = ComplexStruct {
// //         id: 42,
// //         data: "data",
// //         cache: HashMap::new(),
// //         extra: "Extra data".to_string(),
// //     };

// //     // 使用特性方法
// //     complex_struct.cache.insert("key".to_string(), 42);
// //     println!("Cache size: {}", complex_struct.process_data());

// //     // 调用父特性方法
// //     complex_struct.parent_method();
// // }
