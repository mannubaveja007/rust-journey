fn main() {
// in this we will be doing strings in rust which is my love this
//
//
    println!("To bhai yeh hai normal Rust me string");
    let this_is_string : &str = "This is string bro ";
    println!("{}",this_is_string);


    // to dekh bhai 2 types ki strings hai ok 
    // 1. Pointer wala ystem wahi & wala
    // 2. object hota hai ek string ka to koe bi normal string lo aur  usko string ka object dedo
    //    so that will be kind of string 
    //    1. in depth by default static hota hai 
    //
    let my_string : &str = "Toorrr singh dii!!! agee to pta ni jaat daa changa challii jandaa so far !!";
    println!("{}",my_string);
    println!("This was by the &str ok ");

    println!("Now we will explore the same with the object of string");
    // let's do the same with object of the string in the rust ohk!
    //
    let my_string_empty = String::new();
    println!("{}",my_string_empty.len());
    println!("{}",my_string_empty);
    // now we will explore the 2nd object function
    let my_string_filled = String::from("naye je halata na ladya kalla ghare beke mariyna ni galla !");
    println!("{}",my_string_filled);
    println!("{}",my_string_filled.len());
}
