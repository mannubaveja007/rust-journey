// linear search
use std::io;
fn main() {
    let a = [1,2,3,4,5];
    println!("Printing Array {:?}",a);
    let mut x = String::new();
    println!("Enter the number to search :");
    io::stdin().read_line(&mut x).expect("can't understand try again please");
    let result =  linearsearch(a , x);
    println!("Element found at {}",result-1);
    }


fn linearsearch(a : [i32; 5],x : String) -> i32{
    let x : i32 = match x.trim().parse() {
        Ok(x) => x,
        Err(_) => -1, 
    };
    for i in a {
        if i == x {
            return i;
        }
    }
    -1
}