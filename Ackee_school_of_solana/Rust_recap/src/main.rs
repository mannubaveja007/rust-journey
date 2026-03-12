fn longest<'a>(x: &'a str, y: &'a str)->  &'a str {
    if x.len() > y.len() { 
        x
    } else {
        y
}
}



fn main(){
let string1 = String::from("long string is long");
let result;
{
let string2 = String:: from("short");
result = longest(string1.as_str(), string2.as_str()); // Error occurs here!
} // string2 goes out of scope here, but result still holds a reference to it
println! ("The longest string is: {}", result); // Trying to use result after string2 is dropped
}