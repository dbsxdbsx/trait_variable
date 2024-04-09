use std::collections::{BTreeMap, HashSet};
mod utils;
use utils::*;
pub use utils::{CustomType, EnumType};

trait ParentTrait {
    fn parent_trait_method(&self) {
        println!("this is a parent trait method");
    }
}
//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait<T: std::fmt::Debug>: ParentTrait
    where
        T: std::fmt::Display
    {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the TOP of the target trait before any function
        pub i: i32;
        pub b: bool;
            f: f32;
        pub s: String;
            v_i32: Vec<i32>;
            opt_i32: Option<i32>;
        pub tuple : (i32, String, Vec<i32>);
        pub(crate) set_i32: HashSet<i32>;
            bmap: BTreeMap<i32, String>;
        pub(super) custom: CustomType;
        pub(in crate::common) e: EnumType;
            array_i32: [i32; 3];

        // 2. For all valid trait items, like fn, constant values and associated types,
        // they should NOT be above of the trait variables---it is designed on purpose for readability of the target trait
        const VALUE: i32; // constant value
        type Output; // associated type

        // define a method that uses an associated type
        fn test_associated_type_and_constant_value(&self) -> Self::Output;
        // define a method with a generic type
        fn test_generics_with_trait_display(&self, data: T){
            let _i = self.i; // the statement is just to use to test validation of trait variable
            println!("the param data is:{}", data);
        }
        fn test_generics_with_trait_debug(&self, data: T){
            let _i = self.i; // the statement is just to use to test validation of trait variable
            println!("the param data is:{:?}", data);
        }

        fn get_number(&self, num:f32) -> f32 {
            num
        }
        fn get_print_field_b(&self) -> &bool;

        // the below is methods for testing trait variable fields:
        fn test_macro(&self) {
            println!("i32: `{}`, bool: `{}`, f32: `{}`, v_i32: `{:?}`, s:`{}`, opt_i32: `{:?}`, tuple:`{:?}`, set_i32:`{:?}`, btree_map:`{:?}`, custom_type:`{:#?}, enum:`{:?}`, array_i32: `{:?}`", self.i, self.b, self.f, self.v_i32, self.s , self.opt_i32, self.tuple, self.set_i32, self.bmap, self.custom, self.e, self.array_i32); // for macro param `self.i`, it would convert to `*self._x()`
            // println!("i32: `{self.i}`"); // the **Inline Replacement Style** is not supported yet
            eprintln!("i32: `{}`, bool: `{}`, f32: `{}`, v_i32: `{:?}`, s:`{}`, opt_i32: `{:?}`, tuple:`{:?}`, set_i32:`{:?}`, btree_map:`{:?}`, custom_type:`{:#?}`, enum:`{:?}`, array_i32: `{:?}`", self.i, self.b, self.f, self.v_i32, self.s, self.opt_i32, self.tuple, self.set_i32, self.bmap, self.custom, self.e, self.array_i32); // the same as above

            assert!(self.i == self.i);
            assert!(self.i != self.i-1);
            assert!(self.i - 2 != self.i);
            assert_eq!(self.b, self.b);
            assert_ne!(self.f - 1., self.f);
            assert_eq!(self.v_i32, self.v_i32);
            assert_eq!(self.s, self.s);
            assert_eq!(self.opt_i32, self.opt_i32);
            assert_eq!(self.tuple, self.tuple);
            assert_eq!(self.set_i32, self.set_i32);
            assert_eq!(self.bmap, self.bmap);
            assert_eq!(self.custom, self.custom);
            assert_eq!(self.e, self.e);
            assert_eq!(self.array_i32, self.array_i32);
        }

        fn test_assigntment(&mut self) {
            // bak
            let bak_i = self.i.clone();
            let bak_b = self.b;
            let bak_f = self.f;
            let bak_v_i32 = self.v_i32.clone();
            let bak_s = self.s.clone();
            let bak_opt_i32 = self.opt_i32.clone();
            let bak_tuple = self.tuple.clone();
            let bak_set_i32 = self.set_i32.clone();
            let bak_bmap = self.bmap.clone();
            let bak_custom = self.custom.clone();
            let bak_e = self.e.clone();
            let bak_array_i32 = self.array_i32;

            // assignment of i32
            self.i = self.i;
            self.i = 1;
            self.i += 1;
            self.i =  if self.i== 2{
                3 - 3 + self.i
            }else{
                self.i + 2 / 2
            };
            assert!(self.i == 2);

            // assignment of bool
            self.b = true;
            self.b = self.b.clone();
            self.b = !self.b;
            self.b = if self.b == true {
                self.b
            }else{
                !!self.b
            };
            assert_eq!(self.b, false);

            // assignment of f32
            self.f = 3.14;
            self.f *= 0. + self.f - self.get_number(3.14); // ok, the expand logic is the same as `+=`
            assert!(3.14 -(self.get_number(3.14)+ self.f + 0.)<0.01);

            // assignment of Vec<i32>
            self.v_i32 = vec![1, 2, 3];
            self.v_i32.push(4); // this should be converted into `(*self._v_i32_mut()).push(1);`
            self.v_i32[0] += 3 + self.v_i32[1] - self.v_i32[2] * self.v_i32[3];
            assert_eq!(self.v_i32, vec![-6, 2, 3, 4]);

            // assignment of String
            let str = self.s[0..1].to_string();
            assert_eq!(str,"h");

            self.s = "hello".to_string();
            self.s.push_str(" world2");
            assert_eq!(self.s, "hello world2");

            self.s.replace_range(0..5, "Hello");
            assert_eq!(self.s,"Hello world2");

            unsafe {
                let bytes = self.s.as_bytes_mut();
                bytes[6] = b'W';
            }
            assert_eq!(self.s,"Hello World2");
            // assignment of Option<i32>
            self.opt_i32 = Some(1);
            if let Some(i) = self.opt_i32 {
                assert_eq!(i, 1);
                assert_eq!(self.opt_i32, Some(1));
            }
            assert_eq!(self.opt_i32.unwrap(), 1);
            self.opt_i32 = self.opt_i32.clone();
            self.opt_i32 = None;
            assert!(self.opt_i32.is_none());
            // assignment of tuple
            self.tuple.0 = 1;
            self.tuple.1 = "hello".to_string();
            self.tuple.2 = self.v_i32.clone();
            assert_eq!(self.tuple, (1, "hello".to_string(), vec![-6, 2, 3, 4]));
            self.tuple = (2, "world".to_string(), vec![4, 5, 6]);
            assert_eq!(self.tuple, (2, "world".to_string(), vec![4, 5, 6]));
            let (_, ref s, ref v) = self.tuple; // the right side would be converted into `(*self._t())`
            assert_eq!(s, "world");
            assert_eq!(v, &vec![4, 5, 6]);
            let (_, ref  mut s, _) = self.tuple; // the right side would be converted into `(*self._t_mut())`
            assert_eq!(s, "world");
            *s = "world2".into();
            assert_eq!(self.tuple, (2, "world2".to_string(), vec![4, 5, 6]));
            // assignment of HashSet<i32>
            self.set_i32 = HashSet::from([-1,0,1]);
            let new_set =  HashSet::from([0,1,2]);
            let diff_set = HashSet::from([-1, 0, 1, 2]);
            let union_set =  self.set_i32.union(&new_set).copied().collect::<HashSet<_>>();
            assert_eq!(diff_set.difference(&union_set).copied().collect::<Vec<_>>(), vec![]);
            // assignment of BTreeMap<i32, String>
            self.bmap.insert(1, "hello".to_string());
            self.bmap.insert(2, "world".to_string());
            assert_eq!(self.bmap.get(&1), Some(&"hello".to_string()));
            assert_eq!(self.bmap.get(&2), Some(&"world".to_string()));
            // assignment of custom type
            self.custom = CustomType::new();
            self.custom.i = 1;
            assert_eq!(self.custom.i, 1);
            self.i = -1;
            self.custom.i = self.custom.i + self.i;
            assert_eq!(self.custom.i, 0);
            self.custom.str = "hello".to_string();
            assert_eq!(self.custom.str, "hello");
            self.custom.str = self.custom.str.to_uppercase();
            assert_eq!(self.custom.str, "HELLO");
            // assignment of enum type
            self.e = EnumType::Unit;
            assert_eq!(self.e, EnumType::Unit);
            self.e = EnumType::Point { x: 1, y: 2 };
            assert_eq!(self.e, EnumType::Point { x: 1, y: 2 });
            self.e = EnumType::Message("hello".to_string());
            assert_eq!(self.e, EnumType::Message("hello".to_string()));
            self.e = EnumType::Rgb(1, 2, 3);
            assert_eq!(self.e, EnumType::Rgb(1, 2, 3));
            // assignment of array of [i32; 3]
            self.array_i32[0] = 1;
            self.array_i32[1] = 2;
            self.array_i32[2] = 3;
            assert_eq!(self.array_i32, [1, 2, 3]);

            // restore
            self.i = bak_i;
            self.b = bak_b;
            self.f = bak_f;
            self.v_i32 = bak_v_i32;
            self.s = bak_s;
            self.opt_i32 = bak_opt_i32;
            self.tuple = bak_tuple;
            self.set_i32 = bak_set_i32;
            self.bmap = bak_bmap;
            self.custom = bak_custom;
            self.e = bak_e;
            self.array_i32 = bak_array_i32;
        }

        /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test return type↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
        // test return type i32
        fn test_return_ref_i32_by_return_statement(&self) -> &i32{
            // return self.i; // should panic
            return & self.i;
        }
        fn test_return_mut_ref_i32_by_return_statement(&mut self) -> &mut i32{
            // return self.i; // should panic
            // return & self.i; // should panic
            return &  mut self.i;
        }
        fn test_return_ref_i32_by_expression(&self) -> &i32{
            // self.i //should panic
            &self.i
        }
        fn test_return_mut_ref_i32_by_expression(&mut self) -> &mut i32{
            // self.i //should panic
            // &self.i //should panic
            &mut self.i
        }
        fn test_return_cloned_i32_by_explicit_clone_return_statement(&self) -> i32{
            return self.i.clone();
        }
        fn test_return_cloned_i32_by_implicit_clone_return_statement(&self) -> i32{
            return self.i;
        }
        fn test_return_cloned_i32_by_explicit_clone_expression(&self) -> i32{
            self.i.clone()
        }
        fn test_return_cloned_i32_by_implicit_clone_expression(&self) -> i32{
            self.i
        }
        // test return type Vec<i32>
        fn test_return_ref_vec_by_return_statement(&self) -> &[i32]{
            return &self.v_i32;
        }
        fn test_return_mut_ref_vec_by_return_statement(&mut self) -> &mut [i32]{
            return &  mut self.v_i32;
        }
        fn test_return_ref_vec_by_expression(&self) -> &[i32]{
            &self.v_i32
        }
        fn test_return_mut_ref_vec_by_expression(&mut self) -> &mut[i32]{
            &mut self.v_i32
        }
        fn test_return_cloned_vec_by_explicit_clone_return_statement(&self) ->Vec<i32>{
            return self.v_i32.clone();
        }
        fn test_return_cloned_vec_by_explicit_clone_expression(&self) ->Vec<i32>{
            self.v_i32.clone()
        }
        // test return type String, &str
        fn test_return_ref_string_by_return_statement(&self) -> &String{
            return &self.s;
        }
        fn test_return_mut_ref_string_by_return_statement(&mut self) -> &mut String{
            return &mut self.s;
        }
        fn test_return_ref_string_by_expression(&self) -> &String{
            &self.s
        }
        fn test_return_mut_ref_string_by_expression(&mut self) -> &mut String{
            &mut self.s
        }
        fn test_return_cloned_string_by_explicit_clone_return_statement(&self) -> String{
            return self.s.clone();
        }
        fn test_return_cloned_string_by_explicit_clone_expression(&self) -> String{
            self.s.clone()
        }
        fn test_return_ref_str_by_return_statement(&self) -> &str{
            return &self.s;
        }
        fn test_return_ref_str_by_expression(&self) -> &str{
            &self.s
        }
        // test return type Option<i32>
        fn test_return_ref_opt_i32_by_return_statement(&self) -> &Option<i32>{
            return & self.opt_i32;
        }
        fn test_return_mut_ref_opt_i32_by_return_statement(&mut self) -> &mut Option<i32>{
            return &  mut self.opt_i32;
        }
        fn test_return_ref_opt_i32_by_expression(&self) -> &Option<i32>{
            &self.opt_i32
        }
        fn test_return_mut_ref_opt_i32_by_expression(&mut self) -> &mut Option<i32>{
            &mut self.opt_i32
        }
        fn test_return_cloned_opt_i32_by_explicit_clone_return_statement(&self) -> Option<i32>{
            return self.opt_i32.clone();
        }
        fn test_return_cloned_opt_i32_by_implicit_clone_return_statement(&self) -> Option<i32>{
            return self.opt_i32;
        }
        fn test_return_cloned_opt_i32_by_explicit_clone_expression(&self) -> Option<i32>{
            self.opt_i32.clone()
        }
        fn test_return_cloned_opt_i32_by_implicit_clone_expression(&self) -> Option<i32>{
            self.opt_i32
        }
        // test return type tuple (i32, String, Vec<i32>)
        fn test_return_ref_tuple_by_return_statement(&self) -> &(i32, String, Vec<i32>){
            return &self.tuple;
        }
        fn test_return_mut_ref_tuple_by_return_statement(&mut self) -> &mut (i32, String, Vec<i32>){
            return &mut self.tuple;
        }
        fn test_return_ref_tuple_by_expression(&self) -> &(i32, String, Vec<i32>){
            &self.tuple
        }
        fn test_return_mut_ref_tuple_by_expression(&mut self) -> &mut (i32, String, Vec<i32>){
            &mut self.tuple
        }
        fn test_return_cloned_tuple_by_explicit_clone_return_statement(&self) -> (i32, String, Vec<i32>){
            return self.tuple.clone();
        }
        fn test_return_cloned_tuple_by_explicit_clone_expression(&self) -> (i32, String, Vec<i32>){
            self.tuple.clone()
        }
        // test return type HashSet<i32>
        fn test_return_ref_set_i32_by_return_statement(&self) -> &HashSet<i32>{
            return &self.set_i32;
        }
        fn test_return_mut_ref_set_i32_by_return_statement(&mut self) -> &mut HashSet<i32>{
            return &mut self.set_i32;
        }
        fn test_return_ref_set_i32_by_expression(&self) -> &HashSet<i32>{
            &self.set_i32
        }
        fn test_return_mut_ref_set_i32_by_expression(&mut self) -> &mut HashSet<i32>{
            &mut self.set_i32
        }
        fn test_return_cloned_set_i32_by_explicit_clone_return_statement(&self) -> HashSet<i32>{
            return self.set_i32.clone();
        }
        fn test_return_cloned_set_i32_by_explicit_clone_expression(&self) -> HashSet<i32>{
            self.set_i32.clone()
        }
        // test return type BTreeMap<i32, String>
        fn test_return_ref_bmap_by_return_statement(&self) -> &BTreeMap<i32, String>{
            return &self.bmap;
        }
        fn test_return_mut_ref_bmap_by_return_statement(&mut self) -> &mut BTreeMap<i32, String>{
            return &mut self.bmap;
        }
        fn test_return_ref_bmap_by_expression(&self) -> &BTreeMap<i32, String>{
            &self.bmap
        }
        fn test_return_mut_ref_bmap_by_expression(&mut self) -> &mut BTreeMap<i32, String>{
            &mut self.bmap
        }
        fn test_return_cloned_bmap_by_explicit_clone_return_statement(&self) -> BTreeMap<i32, String>{
            return self.bmap.clone();
        }
        fn test_return_cloned_bmap_by_explicit_clone_expression(&self) -> BTreeMap<i32, String>{
            self.bmap.clone()
        }
        // test return type custom type
        fn test_return_ref_custom_by_return_statement(&self) -> &CustomType{
            return &self.custom;
        }
        fn test_return_mut_ref_custom_by_return_statement(&mut self) -> &mut CustomType{
            return &mut self.custom;
        }
        fn test_return_ref_custom_by_expression(&self) -> &CustomType{
            &self.custom
        }
        fn test_return_mut_ref_custom_by_expression(&mut self) -> &mut CustomType{
            &mut self.custom
        }
        fn test_return_cloned_custom_by_explicit_clone_return_statement(&self) -> CustomType{
            return self.custom.clone();
        }
        fn test_return_cloned_custom_by_explicit_clone_expression(&self) -> CustomType{
            self.custom.clone()
        }
        // test return type enum type
        fn test_return_ref_enum_by_return_statement(&self) -> &EnumType{
            return &self.e;
        }
        fn test_return_mut_ref_enum_by_return_statement(&mut self) -> &mut EnumType{
            return &mut self.e;
        }
        fn test_return_ref_enum_by_expression(&self) -> &EnumType{
            &self.e
        }
        fn test_return_mut_ref_enum_by_expression(&mut self) -> &mut EnumType{
            &mut self.e
        }
        fn test_return_cloned_enum_by_explicit_clone_return_statement(&self) -> EnumType{
            return self.e.clone();
        }
        fn test_return_cloned_enum_by_explicit_clone_expression(&self) -> EnumType{
            self.e.clone()
        }
        // test return type array of [i32; 3]
        fn test_return_ref_array_i32_by_return_statement(&self) -> &[i32; 3]{
            return &self.array_i32;
        }
        fn test_return_mut_ref_array_i32_by_return_statement(&mut self) -> &mut [i32; 3]{
            return &mut self.array_i32;
        }
        fn test_return_ref_array_i32_by_expression(&self) -> &[i32; 3]{
            &self.array_i32
        }
        fn test_return_mut_ref_array_i32_by_expression(&mut self) -> &mut [i32; 3]{
            &mut self.array_i32
        }
        fn test_return_cloned_array_i32_by_explicit_clone_return_statement(&self) -> [i32; 3]{
            return self.array_i32.clone();
        }
        fn test_return_cloned_array_i32_by_explicit_clone_expression(&self) -> [i32; 3]{
            self.array_i32.clone()
        }
        /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test return type↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

        /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test param↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
        // test param i32
        fn test_param_i32(&self) {
            assert_eq!(test_with_param_i32(self.i), self.i);
        }
        fn test_ref_param_i32(&self) {
            assert_eq!(test_with_ref_param_i32(&self.i), self.i);
        }
        fn test_mut_ref_param_i32(&mut self) {
           assert_eq!(test_with_mut_ref_param_i32(&mut self.i), self.i);
        }
        // test param vec<i32>
        fn test_param_vec_i32(&self) {
            assert_eq!(test_with_param_vec_i32(self.v_i32.clone()), self.v_i32);
        }
        fn test_ref_param_vec_i32(&self) {
            assert_eq!(test_with_ref_param_vec_i32(&self.v_i32), self.v_i32);
        }
        fn test_mut_ref_param_vec_i32(&mut self) {
           assert_eq!(test_with_mut_ref_param_vec_i32(&mut self.v_i32), self.v_i32);
        }
        // test param String, &str
        fn test_param_string(&self) {
            assert_eq!(test_with_param_string(self.s.clone()), self.s);
        }
        fn test_ref_param_str(&self) {
            assert_eq!(test_with_ref_param_str(&self.s), self.s);
        }
        fn test_mut_ref_param_str(&mut self) {
           assert_eq!(test_with_mut_ref_param_str(&mut self.s), self.s);
        }
        // test param Option<i32>
        fn test_param_opt_i32(&self) {
            assert_eq!(test_with_param_opt_i32(self.opt_i32), self.opt_i32);
        }
        fn test_ref_param_opt_i32(&self) {
            assert_eq!(test_with_ref_param_opt_i32(&self.opt_i32), self.opt_i32);
        }
        fn test_mut_ref_param_opt_i32(&mut self) {
              assert_eq!(test_with_mut_ref_param_opt_i32(&mut self.opt_i32), self.opt_i32);
        }
        // test param tuple (i32, String, Vec<i32>)
        fn test_param_tuple(&self) {
            assert_eq!(test_with_param_tuple(self.tuple.clone()), self.tuple);
        }
        fn test_ref_param_tuple(&self) {
            assert_eq!(test_with_ref_param_tuple(&self.tuple), self.tuple);
        }
        fn test_mut_ref_param_tuple(&mut self) {
              assert_eq!(test_with_mut_ref_param_tuple(&mut self.tuple), self.tuple);
        }
        // test param HashSet<i32>
        fn test_param_set_i32(&self) {
            assert_eq!(test_with_param_set_i32(self.set_i32.clone()), self.set_i32);
        }
        fn test_ref_param_set_i32(&self) {
            assert_eq!(test_with_ref_param_set_i32(&self.set_i32), self.set_i32);
        }
        fn test_mut_ref_param_set_i32(&mut self) {
              assert_eq!(test_with_mut_ref_param_set_i32(&mut self.set_i32), self.set_i32);
        }
        // test param BTreeMap<i32, String>
        fn test_param_bmap(&self) {
            assert_eq!(test_with_param_bmap(self.bmap.clone()), self.bmap);
        }
        fn test_ref_param_bmap(&self) {
            assert_eq!(test_with_ref_param_bmap(&self.bmap), self.bmap);
        }
        fn test_mut_ref_param_bmap(&mut self) {
              assert_eq!(test_with_mut_ref_param_bmap(&mut self.bmap), self.bmap);
        }
        // test param custom type
        fn test_param_custom(&self) {
            assert_eq!(test_with_param_custom(self.custom.clone()), self.custom);
        }
        fn test_ref_param_custom(&self) {
            assert_eq!(test_with_ref_param_custom(&self.custom), self.custom);
        }
        fn test_mut_ref_param_custom(&mut self) {
              assert_eq!(test_with_mut_ref_param_custom(&mut self.custom), self.custom);
        }
        // test param enum type
        fn test_param_enum(&self) {
            assert_eq!(test_with_param_enum(self.e.clone()), self.e);
        }
        fn test_ref_param_enum(&self) {
            assert_eq!(test_with_ref_param_enum(&self.e), self.e);
        }
        fn test_mut_ref_param_enum(&mut self) {
            assert_eq!(test_with_mut_ref_param_enum(&mut self.e), self.e);
        }
        // test param array of [i32; 3]
        fn test_param_array_i32(&self) {
            assert_eq!(test_with_param_array_i32(self.array_i32), self.array_i32);
        }
        fn test_ref_param_array_i32(&self) {
            assert_eq!(test_with_ref_param_array_i32(&self.array_i32), self.array_i32);
        }
        fn test_mut_ref_param_array_i32(&mut self) {
            assert_eq!(test_with_mut_ref_param_array_i32(&mut self.array_i32), self.array_i32);
        }
        /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test param↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

        /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test conditional/loop↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
        fn test_if_else(&mut self) {
            let bak_i = self.i;
            self.i = 5;
            if self.i<0 {
                assert!(self.i<0);
            } else if self.i < 5 {
                assert!(self.i<5);
            }
            else {
                assert!(self.i>=5);
            }
            self.i = bak_i;
        }
        fn test_match_arm(&mut self) {
            // bak
            let bak_i = self.i;
            let bak_opt_i32 = self.opt_i32;
            let bak_e = self.e.clone();

            // test i32
            self.i = 5;
            match self.i {
                0 => unreachable!(),
                1..=4 => unreachable!(),
                5 => assert_eq!(self.i, 5),
                _ if self.i > 5 => unreachable!(),
                _ => unreachable!()

            }
            // test Option<i32>
            self.opt_i32 = Some(2);
            match self.opt_i32 {
                Some(i) => assert_eq!(i, 2),
                None => unreachable!()
            }
            // test enum type
            self.e = EnumType::Message("hello".to_string());
            match &mut self.e {
                EnumType::Message(s) => s.push_str(" world"),
                _ => (),
            }
            assert_eq!(self.e, EnumType::Message("hello world".to_string()));

            if let EnumType::Message(s) = &mut self.e {
                s.push_str("!");
            } else {
                panic!("Expected EnumType::Message, got something else");
            }
            assert_eq!(self.e, EnumType::Message("hello world!".to_string()));

            // restore
            self.i = bak_i;
            self.opt_i32 = bak_opt_i32;
            self.e = bak_e;
        }
        fn test_raw_loop(&mut self) {
            // bak
            let bak_i = self.i;
            let bak_set_i32 = self.set_i32.clone();
            let bak_bmap = self.bmap.clone();
            let bak_array_i32 = self.array_i32;

            // test i32
            self.i = 100;
            let mut j = 0;
            loop {
                if j>=self.i {
                    break;
                }
                assert_eq!(j as i32, j);
                j += 1;
            }
            // test HashSet<i32>
            self.set_i32 = HashSet::from([1, 2, 3]);
            let mut sum = 0;
            let mut iter = self.set_i32.iter();
            loop {
                match iter.next() {
                    Some(i) => sum += i,
                    None => break,
                }
            }
            assert_eq!(sum, 6);
            // test BTreeMap<i32, String>
            self.bmap = BTreeMap::new();
            self.bmap.insert(1, "hello".to_string());
            self.bmap.insert(2, "world".to_string());
            let mut sum = 0;
            let mut vec = Vec::new();
            let mut iter = self.bmap.iter();
            loop {
                match iter.next() {
                    Some((k, v)) => {
                        sum += k;
                        vec.push(v.clone());
                    },
                    None => break,
                }
            }
            assert_eq!(sum, 3);
            assert_eq!(vec, vec!["hello".to_string(), "world".to_string()]);
            // test array of [i32; 3]
            self.array_i32 = [1, 2, 3];
            let mut sum = 0;
            let mut iter = self.array_i32.iter();
            loop {
                match iter.next() {
                    Some(i) => sum += i,
                    None => break,
                }
            }
            assert_eq!(sum, 6);

            // restore
            self.i = bak_i;
            self.set_i32 = bak_set_i32;
            self.bmap = bak_bmap;
            self.array_i32 = bak_array_i32;
        }
        fn test_for_loop(&mut self) {
            // bak
            let bak_i = self.i;
            let bak_set_i32 = self.set_i32.clone();
            let bak_bmap = self.bmap.clone();
            let bak_array_i32 = self.array_i32;

            // test i32
            self.i = 100;
            for (i,j) in (0..self.i).enumerate() {
               assert_eq!(i as i32, j);
            }
            // test HashSet<i32>
            self.set_i32 = HashSet::from([1, 2, 3]);
            let mut sum = 0;
            for i in &self.set_i32 {
                sum += i;
            }
            assert_eq!(sum, 6);
            // test BTreeMap<i32, String>
            self.bmap = BTreeMap::new();
            self.bmap.insert(1, "hello".to_string());
            self.bmap.insert(2, "world".to_string());
            let mut sum = 0;
            let mut vec = Vec::new();
            for (k, v) in &self.bmap {
                sum += k;
                vec.push(v.clone());
            }
            assert_eq!(sum, 3);
            assert_eq!(vec, vec!["hello".to_string(), "world".to_string()]);
            // test array of [i32; 3]
            self.array_i32 = [1, 2, 3];
            let mut sum = 0;
            for i in &self.array_i32 {
                sum += i;
            }
            assert_eq!(sum, 6);

            // restore
            self.i = bak_i;
            self.set_i32 = bak_set_i32;
            self.bmap = bak_bmap;
            self.array_i32 = bak_array_i32;
        }
        /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test conditional/loop↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

        /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test lambda↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
        // for i32
        fn test_lambda_for_i32(&mut self) {
            let bak_i = self.i;

            self.i = 5;
            // lambda with block
            let mut lambda = |delta: i32| {
                self.i += delta;
            };
            lambda(10);
            assert_eq!(self.i, 15);
            // lambda with expression
            let mut lambda = |delta: i32| self.i += delta;
            lambda(10);
            assert_eq!(self.i, 25);

            self.i = bak_i;
        }
        // for Vec<i32>
        fn test_lambda_for_vec_i32(&mut self) {
            let bak_v_i32 = self.v_i32.clone();

            self.v_i32 = vec![1, 2, 3];
            // lambda with block
            let mut lambda = |delta: i32| {
                self.v_i32.push(delta);
            };
            lambda(10);
            assert_eq!(self.v_i32, vec![1, 2, 3, 10]);
            // lambda with expression
            let mut lambda = |delta: i32| self.v_i32.push(delta);
            lambda(10);
            assert_eq!(self.v_i32, vec![1, 2, 3, 10, 10]);

            self.v_i32 = bak_v_i32;
        }
        // for String, &str
        fn test_lambda_for_string_and_str(&mut self) {
            let bak_s = self.s.clone();

            self.s = "hello".to_string();
            // lambda with block
            let mut lambda = |s: &str| {
                self.s.push_str(s);
            };
            lambda(" world");
            assert_eq!(self.s, "hello world");
            // lambda with expression
            let mut lambda = |s: &str| self.s.push_str(s);
            lambda(" world");
            assert_eq!(self.s, "hello world world");

            self.s = bak_s;
        }
        // for Option<i32>
        fn test_lambda_for_opt_i32(&mut self) {
            let bak_opt_i32 = self.opt_i32;

            self.opt_i32 = Some(1);
            // lambda with block
            let mut lambda = |delta: i32| {
                self.opt_i32 = Some(delta);
            };
            lambda(10);
            assert_eq!(self.opt_i32, Some(10));
            // lambda with expression
            let mut lambda = |delta: i32| self.opt_i32 = Some(delta);
            lambda(100);
            assert_eq!(self.opt_i32, Some(100));

            self.opt_i32 = bak_opt_i32;
        }
        // for tuple (i32, String, Vec<i32>)
        fn test_lambda_for_tuple(&mut self) {
            let bak_tuple = self.tuple.clone();

            self.tuple = (1, "hello".to_string(), vec![1, 2, 3]);
            // lambda with block
            let mut lambda = |delta: i32| {
                self.tuple.0 += delta;
            };
            lambda(10);
            assert_eq!(self.tuple, (11, "hello".to_string(), vec![1, 2, 3]));
            // lambda with expression
            let mut lambda = |delta: i32| self.tuple.0 += delta;
            lambda(100);
            assert_eq!(self.tuple, (111, "hello".to_string(), vec![1, 2, 3]));

            self.tuple = bak_tuple;
        }
        // for HashSet<i32>
        fn test_lambda_for_set_i32(&mut self) {
            let bak_set_i32 = self.set_i32.clone();

            self.set_i32 = HashSet::from([1, 2, 3]);
            // lambda with block
            let mut lambda = |delta: i32| {
                self.set_i32.insert(delta);
            };
            lambda(10);
            let expected_set: HashSet<i32> = HashSet::from([1, 2, 3, 10]);
            let difference: HashSet<_> = self.set_i32.difference(&expected_set).cloned().collect();
            assert!(difference.is_empty());
            // lambda with expression
            let mut lambda = |delta: i32| self.set_i32.insert(delta);
            lambda(100);
            let expected_set: HashSet<i32> = HashSet::from([1, 2, 3, 10, 100]);
            let difference: HashSet<_> = self.set_i32.difference(&expected_set).cloned().collect();
            assert!(difference.is_empty());

            self.set_i32 = bak_set_i32;
        }
        // for BTreeMap<i32, String>
        fn test_lambda_for_bmap(&mut self) {
            let bak_bmap = self.bmap.clone();

            self.bmap = BTreeMap::new();
            self.bmap.insert(1, "hello".to_string());
            self.bmap.insert(2, "world".to_string());
            // lambda with block
            let mut lambda = |k: i32, v: String| {
                self.bmap.insert(k, v);
            };
            lambda(3, "hello world".to_string());
            assert_eq!(self.bmap.get(&3), Some(&"hello world".to_string()));
            // lambda with expression
            let mut lambda = |k: i32, v: String| self.bmap.insert(k, v);
            lambda(4, "hello world2".to_string());
            assert_eq!(self.bmap.get(&4), Some(&"hello world2".to_string()));

            self.bmap = bak_bmap;
        }
        // for custom type
        fn test_lambda_for_custom(&mut self) {
            let bak_custom = self.custom.clone();

            self.custom = CustomType::new();
            // lambda with block
            let mut lambda = |prop: i32, str: String| {
                self.custom.i = prop;
                self.custom.str = str;
            };
            lambda(1, "hello".to_string());
            assert_eq!(self.custom.i, 1);
            assert_eq!(self.custom.str, "hello");
            // lambda with expression
            let mut lambda = |prop: i32, str: String| {
                self.custom.i = prop;
                self.custom.str = str;
            };
            lambda(2, "world".to_string());
            assert_eq!(self.custom.i, 2);
            assert_eq!(self.custom.str, "world");

            self.custom = bak_custom;
        }
        // for enum type
        fn test_lambda_for_enum(&mut self) {
            let bak_e = self.e.clone();

            self.e = EnumType::Unit;
            // lambda with block
            let mut lambda = |e: EnumType| {
                self.e = e;
            };
            lambda(EnumType::Point { x: 1, y: 2 });
            assert_eq!(self.e, EnumType::Point { x: 1, y: 2 });
            // lambda with expression
            let mut lambda = |e: EnumType| self.e = e;
            lambda(EnumType::Message("hello".to_string()));
            assert_eq!(self.e, EnumType::Message("hello".to_string()));

            self.e = bak_e;
        }
        // for array of [i32; 3]
        fn test_lambda_for_array_i32(&mut self) {
            let bak_array_i32 = self.array_i32;

            self.array_i32 = [1, 2, 3];
            // lambda with block
            let mut lambda = |i: i32| {
                self.array_i32[0] = i;
            };
            lambda(10);
            assert_eq!(self.array_i32, [10, 2, 3]);
            // lambda with expression
            let mut lambda = |i: i32| self.array_i32[1] = i;
            lambda(100);
            assert_eq!(self.array_i32, [10, 100, 3]);

            self.array_i32 = bak_array_i32;
        }
        /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test lambda↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
    }
}
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
// way1: use the attribute macro to expand the struct (Recommended)
#[trait_var(MyTrait)]
pub struct MyStruct {
    a: i32,
}
// way2: use the hidden declarative macro to expand the struct (Not recommended)
// MyTrait_for_struct! {
//     pub struct MyStruct { // feel free to add `pub` when needed
//      // feel free to add any fields as usual or leave it empty
//      a: i32,
//     }
// }
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct impl↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
impl MyStruct {
    pub fn new(
        a: i32,
        i: i32,
        b: bool,
        f: f32,
        v_i32: Vec<i32>,
        s: &str,
        opt_i32: Option<i32>,
        tuple: (i32, String, Vec<i32>),
        set_i32: HashSet<i32>,
        b_map: BTreeMap<i32, String>,
        custom: CustomType,
        e: EnumType,
        array_i32: [i32; 3],
    ) -> Self {
        Self {
            a,
            i,
            b,
            f,
            v_i32,
            s: s.into(),
            opt_i32,
            tuple,
            set_i32,
            bmap: b_map,
            custom,
            e,
            array_i32,
        }
    }
    pub fn get_explicit_field_a(&self) -> i32 {
        self.a
    }
    pub fn get_impl_trait_field_i(&self) -> i32 {
        self.i
    }
}

// don't forget to implement the original parent trait for the struct
impl ParentTrait for MyStruct {}

impl MyTrait<i32> for MyStruct {
    type Output = i32;
    const VALUE: i32 = 10;

    // impl trait method with associated type
    fn test_associated_type_and_constant_value(&self) -> Self::Output {
        Self::VALUE * self.i
    }

    // impl trait method with generic type
    fn test_generics_with_trait_display(&self, data: i32) {
        println!("the generic type data is: {}", data);
    }

    fn get_print_field_b(&self) -> &bool {
        println!("b: `{}`", self.b);
        &self.b
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct impl↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
