
struct mera_bhai{
    name : String,
    age : u32,
}

fn main() {
    // println!("Hello, world!");
    let merabhai = mera_bhai{
        name : "Mannu Baveja".to_string(), // yeh wo String wala ban gaya ab
        age : 19
    };
    print!("Name ==> {}\nAge ==> {}\n",merabhai.name,merabhai.age)
}
