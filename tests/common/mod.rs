fn test_with_param_i32(i: i32) -> i32 {
    i
}
fn test_with_ref_param_i32(i: &i32) -> i32 {
    *i
}
fn test_with_mut_ref_param_i32(i: &mut i32) -> i32 {
    *i
}
fn test_with_param_vec_i32(v: Vec<i32>) -> Vec<i32> {
    v.clone()
}
fn test_with_ref_param_vec_i32(v: &[i32]) -> Vec<i32> {
    v.to_vec()
}
fn test_with_mut_ref_param_vec_i32(v: &mut [i32]) -> Vec<i32> {
    v.to_vec()
}
//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the TOP of the target trait before any function
            i: i32;
        pub b: bool;
        pub f: f32;
            // s: String;
            v_i32: Vec<i32>;  // add return type test, iter like lambda test...

        // 2.the order of the function definition doesn't matter
        fn get_number(&self, num:f32) -> f32 {
            num
        }
        fn get_print_field_b(&self) -> &bool;

        // the below is methods for testing trait variable fields:
        fn test_macro(&self) {
            println!("i32: `{}`, bool: `{}`, f32: `{}`, v_i32: `{:?}`", self.i, self.b, self.f, self.v_i32); // for macro param `self.i`, it would convert to `*self._x()`
            // println!("i32: `{self.i}`"); // the **Inline Replacement Style** is not supported yet
            eprintln!("i32: `{}`, bool: `{}`, f32: `{}`, v_i32: `{:?}", self.i, self.b, self.f, self.v_i32); // the same as above
            assert!(self.i == self.i);
            assert_eq!(self.b, self.b);
            assert_ne!(self.f+1., self.f);
            assert_eq!(self.v_i32, self.v_i32);
        }

        fn test_assigntment(&mut self) {
            // bak
            let bak_i = self.i.clone();
            let bak_b = self.b;
            // let bak_b = (*self._b_mut());
            let bak_f = self.f;
            let bak_v_i32 = self.v_i32.clone();
            // println!("bak is:{:?}",std::any::type_name_of_val(bak_i));

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
            self.v_i32.push(1); // this should be converted into `(*self._v_i32_mut()).push(1);`
            assert_eq!(self.v_i32, vec![1, 2, 3, 1]);
            // restore
            self.i = bak_i;
            self.b = bak_b;
            self.f = bak_f;
            self.v_i32 = bak_v_i32;
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
        /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test return type↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

        /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test param↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
        // test param i32
        fn test_param_i32(&self) {
            assert_eq!(test_with_param_i32(self.i), self.i);
        }
        fn test_ref_param_i32(&mut self) {
            assert_eq!(test_with_ref_param_i32(&self.i), self.i);
        }
        fn test_mut_ref_param_i32(&mut self) {
           assert_eq!(test_with_mut_ref_param_i32(&mut self.i), self.i);
        }
        // test param vec<i32>
        fn test_param_vec_i32(&self) {
            assert_eq!(test_with_param_vec_i32(self.v_i32.clone()), self.v_i32);
        }
        fn test_ref_param_vec_i32(&mut self) {
            assert_eq!(test_with_ref_param_vec_i32(&self.v_i32), self.v_i32);
        }
        fn test_mut_ref_param_vec_i32(&mut self) {
           assert_eq!(test_with_mut_ref_param_vec_i32(&mut self.v_i32), self.v_i32);
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
            let bak_i = self.i;
            self.i = 5;
            match self.i {
                0 => unreachable!(),
                1..=4 => unreachable!(), // 使用范围匹配来简化代码
                5 => assert_eq!(self.i, 5),
                _ if self.i > 5 => unreachable!(), // 使用匹配守卫来添加额外的条件
                _ => unreachable!()

            }
            self.i = bak_i;
        }
        fn test_raw_loop(&mut self) {
            let bak_i = self.i;
            self.i = 100;
            let mut j = 0;
            loop {
                if j>=self.i {
                    break;
                }
                assert_eq!(j as i32, j);
                j += 1;
            }
            self.i = bak_i;
        }
        fn test_for_loop(&mut self) {
            let bak_i = self.i;
            self.i = 100;
            for (i,j) in (0..self.i).enumerate() {
               assert_eq!(i as i32, j);
            }
            self.i = bak_i;
        }
        /*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑test conditional/loop↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/

        /*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓test lambda↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
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
        fn test_lambda_for_vec_i32(&mut self) {
            let _bak_v_i32 = self.v_i32.clone();
            self.v_i32 = vec![1, 2, 3];
            // lambda with block
            // let mut lambda = |delta: i32| {
            //     self.v_i32.push(delta);
            // };
            // lambda(10);
            // assert_eq!(self.v_i32, vec![1, 2, 3, 10]);
            // // lambda with expression
            // let mut lambda = |delta: i32| self.v_i32.push(delta);
            // lambda(10);
            // assert_eq!(self.v_i32, vec![1, 2, 3, 10, 10]);
            // self.v_i32 = bak_v_i32;
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
//     (_MyTrait) // inputput the hiddent parent trait
//     pub struct MyStruct { // feel free to add `pub` when needed
//      // feel free to add any fields as usual or leave it empty
//      a: i32,
//      pub b: String,
//     }
// }
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct impl↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
impl MyStruct {
    pub fn new(a: i32, i: i32, b: bool, f: f32, v_i32: Vec<i32>) -> Self {
        Self { a, i, b, f, v_i32 }
    }
    pub fn get_print_field_a(&self) -> &i32 {
        println!("a: `{}`", self.a);
        &self.a
    }
}

impl MyTrait for MyStruct {
    fn get_print_field_b(&self) -> &bool {
        println!("b: `{}`", self.b);
        &self.b
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct impl↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
