//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓trait definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
use trait_variable::{trait_var, trait_variable};
trait_variable! {
    pub(crate) trait MyTrait {  // feel free to add `pub` when needed
        // 1.put the variable fields definition at the top of the target trait before any function
            x: i32;
        pub y: bool;
        pub z: f32;

        // 2.the order of the function definition doesn't matter
        fn print_all_trait_fields(&self) {
            println!("x: `{}`, y: `{}`, z: `{}`", self.x, self.y, self.z); // for macro param `self.x`, it would convert to `*self._x()`
            eprintln!("x: `{}`, y: `{}`, z: `{}`", self.x, self.y, self.z); // the same as above
            // println!("x: `{self.x}`"); // the **Inline Replacement Style** is not supported yet
        }
        fn get_print_field_x(&self) -> &i32{
            println!("x: `{}`", self.x);
            return self.x;
        }

        fn return_x_plus_a_num(&self, num: i32) -> i32 {
            return self.x + num;
        }

        fn get_print_field_y(&self) -> &bool;
        fn return_y_clone(&self) -> bool {
            self.y.clone()
        }
        fn get_print_field_z(&self) -> &f32;
        fn change_get_print_field_z(&mut self, ref_z: fn(&f32), ref_z_mut: fn(&mut f32))->&f32 {
            let bak_z = self.z.clone();
            self.z = 4.;
            self.z = if self.z > 0. { -self.z } else { self.z }; // TODO: ok but not complete for complex expression in blocks

            // modify the field by assignment operation
            self.z = 4. + self.z ; // ok, the left `self.z` would convert to `(*self._z_mut())`, and the right `self.z` would convert to `*self._z()`
            self.z += 4. + self.z ; // ok, the left `self.z` would convert to `(*self._z_mut())`, and the right `self.z` would convert to `*self._z()`
            self.z -= 4. + self.z ; // ok, the expand logic is the same as `+=`
            self.z *= 4. + self.z ; // ok, the expand logic is the same as `+=`
            self.z /= 4. + self.z ; // ok, the expand logic is the same as `+=`
            // self.z = 4+ func(&mut self.z); // TODO:

            // modify the field by function call
            // func(&self._x(), self._z_mut()); // ok, the left `self.x` would convert to `*self._x()`, and the right `self.z` would convert to `*self._z_mut()`
            // ref_z(&self.z); // TODO: test with both mutable and immutable reference trait fields
            // ref_z_mut(&mut self.z); // ok, the right `self.z` would convert to `*self._z_mut()`

            // return
            self.z = bak_z;
            self.z
        }

        fn get_cloned_trait_field(&self) -> (i32, bool, f32) {
            (*self.x, *self.y, *self.z)
        }
    }
}
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑trait definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

//↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct definition↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
// way1: use the attribute macro to generate the struct (Recommended)
#[trait_var(MyTrait)]
pub struct MyStruct {
    a: i32,
    pub b: String,
}
// way2: use the hidden declarative macro to generate the struct (Not recommended)
// MyTrait_for_struct! {
//     (_MyTrait) // inputput the hiddent parent trait
//     pub struct MyStruct { // TODO: feel free to add `pub` when needed
//      // feel free to add any fields as usual or leave it empty
//      a: i32,
//      pub b: String,
//     }
// }
//↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct definition↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑

/*↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓struct impl↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓*/
impl MyStruct {
    pub fn new(a: i32, b: String, x: i32, y: bool, z: f32) -> Self {
        Self { a, b, x, y, z }
    }
    pub fn get_print_field_a(&self) -> &i32 {
        println!("a: `{}`", self.a);
        &self.a
    }
    pub fn get_print_field_b(&self) -> &String {
        println!("b: `{}`", self.b);
        &self.b
    }
}

impl MyTrait for MyStruct {
    fn get_print_field_y(&self) -> &bool {
        println!("y: `{}`", self.y);
        &self.y
    }
    fn get_print_field_z(&self) -> &f32 {
        println!("z: `{}`", self.z);
        &self.z
    }
}
/*↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑struct impl↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑*/
