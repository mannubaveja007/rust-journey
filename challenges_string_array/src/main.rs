use std::io;

fn count_upper_lower(s : String) -> (String, String, String, String) {
    let mut countU : i32 = 0;
    let mut countL : i32 = 0;
    let mut countD : i32 = 0;
    let mut countS : i32 = 0;
    for ch in s.chars(){
        if ch.is_uppercase(){
            countU= countU+1;
        }
        else if ch.is_lowercase() {
            countL= countL+1;
        }
        else if ch.is_numeric(){
            countD= countD+1;
        }
        else{
            countS= countS+1;
        }
    }
    return (countU.to_string(),countL.to_string(),countD.to_string(),countS.to_string());
}


fn iterate_over_string(s : String) -> String{
    let mut _new_string = String::new();
    for ch   in s.chars(){
        if ch.is_numeric(){
            continue;
        }
        else{
                _new_string.push(ch);
        }
    }
    return _new_string;
}
fn main() {
//    1. Remove All Digits from a String

    // 1. make a string. iterate . if . push to new . return new

    // let mut _str = String::new();
    // println!("Enter your string: ");
    // io::stdin().read_line(&mut _str).expect("can't read");
    // let main_result = iterate_over_string(_str.trim().to_string());
    // println!("Result ==>{}",main_result);


    // problem 1 is done 

    // 2. Count Uppercase, Lowercase, Digits, and Special Characters
    // Input: "He11o@W0rld!"
    
    let mut _str = String::new();
    println!("Enter your string: ");
    io::stdin().read_line(&mut _str).expect("can't read");
    println!("Working on it!");
    let (U , L , N , S) = count_upper_lower(_str.trim().to_string());
    println!("Uppercase ==> {}",U);
    println!("Lowercase ==> {}",L);
    println!("Numeric ==> {}",N);
    println!("Symbols ==> {}",S);

}
