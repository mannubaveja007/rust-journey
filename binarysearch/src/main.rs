// binary search !


use std::io;
fn main() {
    let a = [1,5,2,5,7,3,45,8]; 
    println!("Printing arr {:?}",a);
    println!("Enter the element to search : ");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Hello world");
    let parsed_int  = match x.trim().parse(){
        Ok(x) => x,
        Err(_) => -1,
    };
    binarysearch(a,parsed_int);

}

fn binarysearch(a : [i32 ; 8] , parsed_int : i32) {
    let n : usize = a.len();
    let mut left = 0;
    let mut right = n-1;
    let mut found : bool = false;
    while left <= right{
        let mid = (left + right) /2;
        if a[mid] == parsed_int{
            println!("Element found at {}",mid);
            found = true;
            break;
        }
        else if a[mid] < parsed_int {
            left = mid+1;
        }
        else{
            right = mid-1;
        }

    }
    if !found {
        println!("Element not found");
    }
}