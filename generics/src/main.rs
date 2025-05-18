use std::io;
fn main() {
    println!("Enter your number one " );
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("camn't understand");
    let mut num2 = String::new();
    println!("Enter your number second ");
    io::stdin().read_line(&mut num2).expect("can't understand");
    let num1_parsed : u32= match num1.trim().parse(){
        Ok(num1) => num1,
        Err(_) => {
            println!("Error while parsing the number");
            return;
        },
    };

    let num2_parsed : u32 = match num2.trim().parse(){
        Ok(n) => n,
        Err(_) => {
            println!("Unable to parse the number");
            return;
        },
    };

    // let result = sum_u32(num1_parsed, num2_parsed);
    // println!("The Result is {}",result);
}
// sum_u32
// sum_f32

fn sum_all<M>(a : M , b : M) -> M {
    return a+b;
}
// fn sum_u32(a : u32 , b : u32 ) -> u32 {
//     a+b
// }
// fn sum_f32(a : f32 , b : f32 ) -> f32 {
//     a+b
// }