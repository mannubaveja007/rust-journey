use core::str;

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize)]

#[derive(Debug)]
struct User{
    username :String,
    is_valid : bool,
    is_active : bool,
    balance : u64,
    password : String,
}
fn main() {
    // println!("Hello, world!");
    // defining a struct

    let u1 = User{
        username : String::from("Mannu"),
        is_active : true,
        is_valid : true,
        balance : 12000000,
        password : String::from("Welcome Bhai Code Base me")
    };
    // println!("{:?}",u1);
    // println!("{}",u1.balance);

    let parsed_json = serde_json::to_string(&u1).expect("can't");
    // match parsed_json {
    //     Ok(str) => println!("{}",str),
    //     Err(_) => println!("Noi :("),
    // }

    println!("{}",parsed_json);
    // println!("{}",parsed_json);

}
