use std::io;

fn check(_str :String) -> i32{
    let mut counter : i32 = 0;
    for ch in _str.chars(){
        if ch=='a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' || ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O'{
            counter = counter+1;
    }
}
    return counter;
}

fn len(_str : String) -> i32{
    let mut counter : i32 = -1; // becuz it's taking the null character also 
    for _ch in _str.chars(){
        counter = counter +1
    }
    return counter;
}
fn skip_digit(_str : String) -> String {
    let mut _newstr = String::new();
    for  ch in _str.chars(){
        if ch.is_numeric(){
            continue;
        } 
        else{
            _newstr.push(ch);
        }
    }
    return _newstr;
}
fn skipM(_str  : String) -> String  {
    let mut _newstr = String::new();
    for mut _ch in _str.chars(){
        if _ch == 'M' || _ch == 'm'{
            continue;
    } else{
        _newstr.push(_ch);
    }
}
return _newstr;
}
fn main() {
    let mut _str = String::new();    
    // we have to take input from a string and count the vowels 
    println!("Enter your string :");
    io::stdin().read_line(&mut _str).expect("Can't read");
    // println!("{}",_str); 
    let len : i32 = len(_str.trim().to_string().clone());
    println!("The Length of your strings is ==> {}",len);
    let result_vowels = check(_str.trim().to_string());
    println!("The total number of vowels in your string ==> {}",result_vowels);
    let mut skip = skipM( _str.clone());
    println!("The String in which M is skipped is {} " ,skip );
    let skipdigit = skip_digit( _str.clone() );
    println!("Here we have skipped the all the numbers from {} to this \n {}",_str , skipdigit);
}
