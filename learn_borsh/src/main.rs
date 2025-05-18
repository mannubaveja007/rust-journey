// use std::fmt::Debug;
use borsh::{BorshDeserialize,BorshSerialize};

#[derive(BorshSerialize,BorshDeserialize)]
#[derive(Debug)]

struct User {
    username : String,
    password : String,
    age : u8
}
fn main() {
    // println!("Hello, world!");
    let u1 = User{
        username : String::from("mannsec"),
        password : String::from("abcd"),
        age : 19,
    };
    let mut v1: Vec<u8> = Vec::new();
    // let borch_byte = BorshDeserialize::deserialize();
    u1.serialize(&mut v1).expect("can't man");
    // println!("Struct \n==>{:?}",u1);
    println!("{:?}",v1);
    println!("Let's DeSerialize it now! 😎");
    let deserial: User = BorshDeserialize::try_from_slice(&v1).expect("can't understand");
    println!("{:?}",deserial);
}
