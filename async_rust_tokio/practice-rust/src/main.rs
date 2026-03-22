// #[derive(Debug)]
// struct User{
//     name : String,
//     age : u32,
//     IsActive : bool
// }


// impl User {
//     fn greet(&self) {
//         println!("Hii , I 'm {}!",self.name);
//     }
// }
// fn main(){
//     let u1 = User {
//         name : String::from("Mannu"),
//         age : 21,
//         IsActive : true,
//     };

//     println!("{:#?}",u1);
//     User::greet(&u1);
// }

// struct Rectangle {
//     width : f64,
//     height : f64
// }

// impl Rectangle {
//     fn is_square(&self) -> bool{
//         if self.width == self.height{
//             return true
//         }
//     return false
//     }
// }
// fn main(){
//     let r1 = Rectangle {
//         height : 150.0,
//         width : 10.0
//     };

//     let store = Rectangle::is_square(&r1);
//     println!("{}",store)
// }


// fn parse_and_add(a: &str,b : &str) -> Result<i64 , String> {
//     let a_num = a.parse::<i64>().map_err(|_| format!("Invalid Number : {}",a))?;
//     let b_num = b.parse::<i64>().map_err(|_| format!("Invalid Number : {}",b))?;
//     return Ok(a_num + b_num)
// }   

// fn main(){
//     let result = parse_and_add("10","20");
//     match result{
//         Ok(a) => println!("{}",a),
//         Err(e) => println!("{:?}",e)
//     }
// }

// fn parse_and_divide(a : &str , b : &str) -> Result<i64 , String> {
//     let a1 = a.parse::<i64>().map_err(|_| format!("Invalid Number : {}",a))?;
//     let b1 = b.parse::<i64>().map_err(|_| format!("Invalid Number : {}" ,b ))?;
//     return Ok(a1/b1)
// }

// fn main(){
//     let result = parse_and_divide("1","10");
//     println!("{:?}",result);
// }



// fn main(){
//     let double = |&x : &str| -> &str {&x};
//     println!("{:?}",double("Mannu"));
// }


// fn main(){
// let words = vec!["hello", "world", "rust", "is", "fast", "and", "awesome"];


// filters words longer than 3 characters

// let result : Vec<&str> = words.clone().into_iter().filter(|x| x.len() > 3).collect();
// println!("{:?}",result);

// let result1 : Vec<String> = words.into_iter().map(|x| x.to_uppercase()).collect();
// println!("{:?}",result1);

// let result2 : Vec<String> = words.into_iter().filter(|x| x.len()).collect();
// println!("{:?}",result2);

// words.into_iter().filter(|x| *x =='a' || *x =='e' || *x =='i' || *x =='o' || *x =='u').collect();


//  let vowels: Vec<&str> = words
//       .into_iter()
//       .filter(|x| ["a", "e", "i", "o", "u"].contains(x))
//       .collect();
//       println!("{:?}",vowels);
// }   

// trait GreetTrait {
//     fn hello(&self) -> String;
// }

// struct Human {name:String}
// struct Robot {id : u32}

// impl GreetTrait for Human{
//     fn hello(&self) -> String{
//         return format!("Hii, {}",self.name)
//     }
// }

// impl GreetTrait for Robot {
//     fn hello(&self) -> String{
//         return format!("Your ID : {}",self.id)
//     }
// }
// fn make_greet(g : &impl GreetTrait) {
//     println!("{}" , g.hello())
// }

// fn main(){
//     make_greet(&Human{name : String::from("Mannu")});
//     make_greet(&Robot{id : 10});

// }   

// trait Area {
//     fn area(&self) -> f64;
// }

// struct Circle {Radius : f64}
// struct Triangle { base : f64,height: f64}

// impl Area for Circle{
//     fn area(&self) -> f64{
//         return 3.17 * self.Radius * self.Radius;
//     }
// }

// impl Area for Triangle{
//     fn area(&self) -> f64{
//         return (self.base * self.height)/2.0;
//     }
// }

// fn print_area(shape: &impl Area) {
//     println!("{}", shape.area());  // just call .area() on what was passed in!
// }

// fn main(){
//     let c = Circle {Radius : 10.0};
//     let r = Triangle{base : 10.0 , height : 10.0};

//     print_area(&c);
//     print_area(&r);
// }


// enum Command {
//     Move {x : i32 , y : i32},
//     Print(String),
//     Quit
// }

// fn run_command(cmd : Command){
//     match cmd{
//         Command::Move {x,y} => println!("Moving to ({},{})", x, y),
//         Command::Print(s) => println!("{}",s),
//         Command::Quit => println!("Quitting")
//     }
// }

// fn main(){
//     run_command(Command::Move {x : 10, y : 20});
//     run_command(Command::Print("Hello".to_string()));
//     run_command(Command::Quit);
// }

// 1. Sequentials - waits for each one before starting next
task_a().await;
task_b().await;

// 2. Concurrent - both run at same time wait for both to finish

tokio::join(task_a(),task_b());


// 3. Spawn - Fire and forget in background ( an children pid is generated)
tokio::spawn(async {task_a().await});

