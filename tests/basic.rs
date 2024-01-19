use trait_variable::trait_variable;

#[trait_variable]
trait MyTrait {
    // let x: i32;
    fn trait_print(&self);
    // fn trait_print2(&self) {
    //     self.field1 = true;
    //     println!("the field1 is {}", self.field1);
    // }
}

#[trait_variable(MyTrait)]
struct MyStruct {
    my_field: i32,
}

impl MyStruct {
    fn self_print(&self) {
        println!("self_print: my_field is {}", self.my_field);
    }
}

impl MyTrait for MyStruct {
    fn trait_print(&self) {
        println!("trait print: my_field is {}", self.my_field);
    }
}

fn main() {
    let my_struct = MyStruct { my_field: 10 };
    my_struct.self_print();
    my_struct.trait_print();
    // my_struct.trait_print2();
}
