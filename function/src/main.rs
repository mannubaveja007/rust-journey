












fn main() {
    let age : i8 = 19;
    let height = 5.9;
    let my_string = "Baveja was here!";
    // let my_string2 = "Baveja was not here";

    println!("{}",my_string);
    println!("Hello Mr. Baveja");
    println!("Mannu's Age is ==> {}",age);
    println!("Mannu's Height is ==> {}",height);

    // strings !
    //
    // Object one --> String:: new , from 
    // &str 
    let my_home_state : &str ="Chandigarh";
    println!("Home . {}",my_home_state);


    // see the object method!
    //
    // String --> object
    // 2 functions --> new and from 
    //
    // new function creates a empty string 
    // from you can create your string
    //
    let my_country = String::new();
    println!("{}",my_country);
    println!("{}",my_country.len());
    // from here!
    //
    let my_country2 = String::from("INDIA");
    println!("{}",my_country2);
    println!("Len --> {}",my_country2.len());
    let my_str = 1337;
    println!("{}",my_str.to_string());
//print_type_of(my_str);
    
}
