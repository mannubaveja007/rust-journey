use std::io;

struct User{
    first_name : String,
    last_name : String,
    money : i64,
    health : u16,
    is_alive : bool,
}

fn main() {
    
    println!("Enter your FirstName : ");
    let mut firstname   = String::new();
    io::stdin().read_line(&mut firstname).expect("can't understand");
    println!("Enter your LastName : ");

    let mut lastname = String::new();
    io::stdin().read_line(&mut lastname).expect("can't understand");

    let p1 = User{
        first_name :firstname.trim().to_string(),
        last_name : lastname.trim().to_string(),
        money : 10000,
        health : 100,
        is_alive :true
    };
    println!("Your name is {} {}  , your health is {} and you have {}$ and you are {}",p1.first_name ,p1.last_name ,p1.health  , p1.money , p1.is_alive);
    // let somenumber : i32 = somestring.trim().parse().expect("not a number");
    // let result = is_even(somenumber);
    // println!("even ? : {}",result);
}


// fn is_even(number : i32) -> bool {
//     if number%2 ==0 {
//         return true;
//     }
//     else{
//         return false;
//     }
// }