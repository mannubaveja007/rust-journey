use std::fs::File;
use std::io::Write;
use std::io;
fn main() {
    // Create a file
    let mut data_file = File::create("data.txt").expect("creation failed");
    let mut f = File::open("data.txt").expect("failed");
    let mut upr = String::new();
    io::stdin().read_line(&mut upr).expect("Hello world");
    // Write contents to the file
    data_file.write(upr.as_bytes()).expect("write failed");
    println!("Created a file data.txt");
}