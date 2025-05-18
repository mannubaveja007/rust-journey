fn main() {
    print_variable(String::from("Hello"));
    print_variable(1);
    // print_variable(1.0);
    biggest_element(1,2);
    biggest_element(10,2);


}

fn print_variable<T: std::fmt::Display>(a : T){
    println!("{}",a)
}


fn biggest_element<T:Ord>(a : T , b : T){
    if a>b{
        println!("A is greater than b");
    }
    else{
        println!("b is greater than ");
    }
}