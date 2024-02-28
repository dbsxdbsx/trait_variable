//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the TOP of the target trait before any function
            i: i32;
        pub b: bool;
        pub f: f32;
            // s: String;
            // v: Vec<i32>;

        // 2.the order of the function definition doesn't matter
        fn get_number(&self, num:f32) -> f32 {
            num
        }

        fn get_print_field_b(&self) -> &bool;

        // the below is methods for testing trait variable fields:
        fn test_macro(&self) {
            println!("i32_field: `{}`, bool_field: `{}`, f32_field: `{}`", self.i, self.b, self.f); // for macro param `self.i`, it would convert to `*self._x()`
            // println!("i32_field: `{self.i}`"); // the **Inline Replacement Style** is not supported yet
            eprintln!("i32_field: `{}`, bool_field: `{}`, f32_field: `{}`", self.i, self.b, self.f); // the same as above
            assert!(self.i == self.i);
            assert_eq!(self.b, self.b);
            assert_ne!(self.f+1., self.f);
        }

        fn test_assigntment(&mut self) {
            // bak
            let bak_i = self.i.clone();
            let bak_b = self.b;
            // let bak_b = (*self._b_mut());
            let bak_f = self.f;
            // println!("bak is:{:?}",std::any::type_name_of_val(bak_i));

            // test
            self.i = self.i;
            self.i = 1;
            self.i += 1;
            assert!(self.i == 2);

            self.b = true;
            self.b = self.b.clone();
            self.b = !self.b;
            assert_eq!(self.b, false);

            self.f = 3.14;
            self.f *= 0. + self.f - self.get_number(3.14); // ok, the expand logic is the same as `+=`
            assert!(3.14 -(self.get_number(3.14)+ self.f + 0.)<0.01);

            // restore
            // self.i = 5;
            self.i = bak_i;
            self.b = bak_b;
            self.f = bak_f;
        }

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

        // fn get_print_field_z(&self) -> &f32;


        // fn get_cloned_trait_field(&self) -> (i32, bool, f32) {
        //     (*self.i, *self.b, *self.f)
        // }

        // fn get_mut_trait_field_x(&mut self) ->&mut i32 {
        //     &mut self.i
        // }

        // fn get_mut_trait_field(&mut self) -> (&mut i32, &mut bool, &mut f32) {
        //     (self.i, self.b, self.f)
        // }
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
    pub fn new(a: i32, i: i32, b: bool, f: f32) -> Self {
        Self { a, i, b, f }
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
