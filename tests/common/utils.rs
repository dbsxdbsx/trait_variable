use std::collections::{BTreeMap, HashSet};

// customized type used as a trait variable field
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CustomType {
    pub i: i32,
    pub str: String,
}

impl CustomType {
    pub fn new() -> Self {
        Self {
            i: 0,
            str: "".to_string(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EnumType {
    Unit,
    Point { x: i32, y: i32 },
    Message(String),
    Rgb(i32, i32, i32),
}
/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓assistant fns↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
// for param (ref of)i32
pub fn test_with_param_i32(i: i32) -> i32 {
    i
}
pub fn test_with_ref_param_i32(i: &i32) -> i32 {
    *i
}
pub fn test_with_mut_ref_param_i32(i: &mut i32) -> i32 {
    *i
}
// for param (ref of)vec<i32>
pub fn test_with_param_vec_i32(v: Vec<i32>) -> Vec<i32> {
    v.clone()
}
pub fn test_with_ref_param_vec_i32(v: &[i32]) -> Vec<i32> {
    v.to_vec()
}
pub fn test_with_mut_ref_param_vec_i32(v: &mut [i32]) -> Vec<i32> {
    v.to_vec()
}
// for param (ref of)String
pub fn test_with_param_string(s: String) -> String {
    s
}
pub fn test_with_ref_param_str(s: &str) -> String {
    s.into()
}
pub fn test_with_mut_ref_param_str(s: &mut str) -> String {
    s.into()
}
// for param (ref of)Option<i32>
pub fn test_with_param_opt_i32(opt: Option<i32>) -> Option<i32> {
    opt
}
pub fn test_with_ref_param_opt_i32(opt: &Option<i32>) -> Option<i32> {
    *opt
}
pub fn test_with_mut_ref_param_opt_i32(opt: &mut Option<i32>) -> Option<i32> {
    *opt
}
// for param (ref of)tuple (i32, String, Vec<i32>)
pub fn test_with_param_tuple(t: (i32, String, Vec<i32>)) -> (i32, String, Vec<i32>) {
    t
}
pub fn test_with_ref_param_tuple(t: &(i32, String, Vec<i32>)) -> (i32, String, Vec<i32>) {
    t.clone()
}
pub fn test_with_mut_ref_param_tuple(t: &mut (i32, String, Vec<i32>)) -> (i32, String, Vec<i32>) {
    t.clone()
}
// for param (ref of)HashSet<i32>
pub fn test_with_param_set_i32(set: HashSet<i32>) -> HashSet<i32> {
    set
}
pub fn test_with_ref_param_set_i32(set: &HashSet<i32>) -> HashSet<i32> {
    set.clone()
}
pub fn test_with_mut_ref_param_set_i32(set: &mut HashSet<i32>) -> HashSet<i32> {
    set.clone()
}
// for param (ref of)BTreeMap<i32, String>
pub fn test_with_param_bmap(bmap: BTreeMap<i32, String>) -> BTreeMap<i32, String> {
    bmap
}
pub fn test_with_ref_param_bmap(bmap: &BTreeMap<i32, String>) -> BTreeMap<i32, String> {
    bmap.clone()
}
pub fn test_with_mut_ref_param_bmap(bmap: &mut BTreeMap<i32, String>) -> BTreeMap<i32, String> {
    bmap.clone()
}
// for param (ref of)custom type
pub fn test_with_param_custom(custom: CustomType) -> CustomType {
    custom
}
pub fn test_with_ref_param_custom(custom: &CustomType) -> CustomType {
    custom.clone()
}
pub fn test_with_mut_ref_param_custom(custom: &mut CustomType) -> CustomType {
    custom.clone()
}
// for param (ref of)enum type
pub fn test_with_param_enum(e: EnumType) -> EnumType {
    e
}
pub fn test_with_ref_param_enum(e: &EnumType) -> EnumType {
    e.clone()
}
pub fn test_with_mut_ref_param_enum(e: &mut EnumType) -> EnumType {
    e.clone()
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑assistant pub fns↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
