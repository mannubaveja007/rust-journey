use std::io;

fn checkforvowels(input : &str){
    let mut counterforvowels : u32 = 0;
    for ch in input.chars(){
        if ch == 'a' || ch == 'e' || ch =='i'|| ch == 'o' || ch == 'u' {
            counterforvowels= counterforvowels+1;
        }
    }
    println!("Number of vowels ==> {}",counterforvowels);
}
fn main() {
    println!("Enter your name : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("CHud gye Guru Moye Moye");
    println!("So we have taken our user input {}" , input);
    // checkforvowels(&input);
    let mut _a = String::new();
    let mut _b = String::new();
    println!("Enter the variable of A : ");
    io::stdin().read_line( &mut _a).expect("You have entered an wrong number");
    println!("Enter the variable of B : ");
    io::stdin().read_line( &mut _b).expect("You have entered the wrong the B");
    println!("SO we are going to make our own calculator ok here only");
    println!("Let's justtt go ahead and take the user input of the these 2 number and then calculate the shit out of here in rust");
    let  a : i32= _a.trim().parse().expect("Not interger");
    let  b : i32 = _b.trim().parse().expect("can't parse the numebr from the input");
    println!("Addition is {}",a+b);
    println!("Substraction is {}",a-b);
    println!("Multiplicaion is {}",a*b);
    println!("Division is {}",a/b);

    
}
