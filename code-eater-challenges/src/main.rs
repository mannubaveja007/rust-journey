fn main() {
    println!("Hello, world!");

    let s1 :String = get_string();
    println!("This is s1: {}",s1);

    let s2:String = String::from("World");
    let s3 : String = send_get_string(s2);

println!("This is s3: {}", s3);
}

fn get_string() -> String {
    let new_string = String::from("hello");
    return new_string;
}

fn send_get_string(rec_string:String) -> String{
    return rec_string;;
}
