enum Shape{
    Circle(i32),
    Rectangle(f32,f32),
    Square(u32),
}

impl Shape {
    fn area(&self){
        match self {
            Shape::Circle()
        }
    }
}


fn main() {
    println!("Hello, world!");
}
