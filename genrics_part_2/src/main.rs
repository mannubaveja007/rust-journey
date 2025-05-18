use std::io;
fn main() {
    println!("Enter the number 1 ");

    let mut num1 = String::new();
    
    io::stdin().read_line(&mut num1).expect("can't understand");
    let num1_parsed : i32 = match num1.trim().parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Can't bro");
            return;
        },
    };
    println!("Enter the number 2");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("can't understand");
    let num2_parsed : i32 = match num2.trim().parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Can't bro");
            return;
        },
    };
    print_variable(num1_parsed);
    print_variable(num2_parsed);


}


fn print_variable<T : std::fmt::Display>(a : T) {
    println!("Variable was ==> {}",a);
}