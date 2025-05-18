use std::io;
use rand::Rng;


fn main() {
    let  guess_arr = ["grapes","mango","apple","guava","pineapple"];
    let mut rng = rand::thread_rng();
    
    let mut choice: String = String::new();

    loop {
    let num: usize = rng.gen_range(0..5);
    // println!("Selected fruit is : {}", guess_arr[num]); // stealth mode
    println!("out of all of these {:?}",guess_arr);
    println!("Guess the fruit : ");
    choice.clear();
    io::stdin().read_line(&mut choice).expect("try again can't understand sorry");
    let guess_it_and_crack_it  = choice.trim();

    match guess_it_and_crack_it{
        x if x == guess_arr[num] => {
            println!("Congratulations you found the flag {{Y0u haVe Pw0ned!}}");
            break;  
        },
        _ => println!("try again babes!")
    }
    }

}
