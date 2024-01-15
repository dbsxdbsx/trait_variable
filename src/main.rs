use std::io;

fn main() {
    println!("Hello, world!");
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("get input failed");
}