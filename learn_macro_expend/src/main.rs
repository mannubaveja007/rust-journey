macro_rules! my_own_macro {
    () => {
        println!("Hey There so I have made my own macro now! 😊🚀");
    }; // ($) => {
       //     println!("hrfugdh");
       // }
}

#[derive(Debug)]

struct User {
    username: String,
    password: String,
    age: u8,
}
fn main() {
    println!("Hello, world!");
    my_own_macro!();
    let u1 = User {
        username: String::from("Mannu Baveja"),
        password: String::from("Pas0wOrd"),
        age: 19,
    };
    println!("Printing my own made struct {}", u1.username);
    println!("Printing my own made struct {}", u1.password);
    println!("Printing my own made struct {}", u1.age);
    // hello world
    // let's print it using debug trait
    // hello world
    println!("Printing the whole Struct at once {:?}", u1);
}
