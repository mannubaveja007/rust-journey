use std::io;

fn main() {
    let mut ame = String::new();
    println!("Enter your name : ");
    io::stdin().read_line(&mut ame).expect("You have entered an wrong value");
    // println!("{}",ame.chars().rev().collect::<String>());
    // let len : i32 = findlen(ame.trim().to_string());
    // println!("The length of the string is {} ",len);
    let reverse_string : String = reverse_string(ame.trim().to_string());
    println!("Your reversed String is ==> {}",reverse_string);
}



fn reverse_string(name : String) -> String{
    let mut _new_string = String::new();
    for ch in name.chars().rev(){
        _new_string.push(ch);
    }
    return _new_string;
}



// fn findlen(name : String) -> i32{
//     let mut _new_string = String::new();
//     let mut len : i32 = 0;
//     for ch in name.chars(){
//         len+=1;
//     }
//     return len;
// }
