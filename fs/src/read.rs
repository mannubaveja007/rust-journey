use std::{fs::File, io::Read};
fn main() {
    // println!("Hello, world!");
    // let mut f = File::open("hello.txt").unwrap();

    let mut f  = File::open("hello.txt").expect("msg");
    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    f.read_to_string(&mut file_content).unwrap();

    println!("{:?}", file_content);
    
}
