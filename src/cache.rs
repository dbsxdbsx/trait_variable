use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use syn::{Ident, Item, ItemStruct, ItemTrait};

// 创建一个新的类型别名，用于存储特性和结构的定义
type DefinitionCache = HashMap<String, Item>;

// lazy_static! {
//     static ref CACHE: Mutex<DefinitionCache> = Mutex::new(HashMap::new());
// }
use once_cell::sync::Lazy;

// static CACHE: Lazy<Mutex<DefinitionCache>> = Lazy::new(|| Mutex::new(HashSet::new()));
// use std::cell::RefCell;
// thread_local!(static CACHE: RefCell<DefinitionCache> = RefCell::new(HashMap::new()));

// fn add_string_if_empty(s: String) -> Vec<String> {
//     CACHE.with(|data| {
//         let mut vec = data.borrow_mut();
//         if vec.is_empty() {
//             vec.push(s);
//         }
//         vec.clone()
//     })
// }

// // 将特性存储到缓存中
// pub fn cache_trait(name: &Ident, trait_item: &ItemTrait) {
//     let mut cache = CACHE.lock().unwrap();
//     cache.insert(name.to_string(), Item::Trait(trait_item.clone()));
// }

// // 从缓存中获取特性
// pub fn get_trait(name: &Ident) -> Option<Item> {
//     let cache = CACHE.lock().unwrap();
//     cache.get(&name.to_string()).cloned()
// }
