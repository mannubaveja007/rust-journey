fn main() {
    // println!("Hello, world!");

    let str1 = String::from("Mannu");
    let str2 = String::from("Baveja");
    // let mut ans = String::new();
    let ans = longest_string(&str1,&str2);
    println!("The Longest String is {} ",ans);
    println!("Life is good!😂")
    
    
}


fn longest_string<'a>(s1 : &'a String , s2  : &'a String) ->  &'a String{
    if s1.len() > s2.len(){
        return s1;
    }
    else {
        return s2;
    }
}