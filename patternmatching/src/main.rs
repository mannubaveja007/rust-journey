use std::io;
fn main() {
    // println!("Enter your age :");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("can't understand");
    // let input =  input.trim();
    // let mut result = match input {
    //     "19" => "You can vote GentleMan",
    //     _ => "You can't baby",
    // };
    // println!("{}",result);


    println!("Enter your age : ");
    let mut age_in = String::new(); 
    io::stdin().read_line(&mut age_in).expect("Can't understand"); // read as a string
    let age_in = age_in.trim(); // remove whitespace \n
    let age : u32 = age_in.parse().expect("can't understand"); // convert it to int 
    match age{
        age if age >19 => println!("hello you can do the vote ok"),
        age if age <19 => println!("Sorry you can't vote"),
        _ => println!("You are Gay!"),
    };

    
}
