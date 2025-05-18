fn main() {
    let mut str = String::from("Mannu");
    let ref1 = &mut str;
    ref1.push_str("Baveja");
    println!("{}",ref1);
    let ref2 = &str;
    let ref3 = &str;
    let ref4 = &str;

    println!("{} , {} , {} ",ref2,ref3,ref4);
}
