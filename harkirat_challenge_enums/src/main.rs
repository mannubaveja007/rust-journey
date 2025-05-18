enum Shape {
    Circle(f32),
    Rectangle(f32,f32),
    Square(u32),
}

impl Shape {
    fn area(&self) -> f32{
        match self {
            Self::Circle(r ) => 3.14 * r * r,
            Self::Rectangle(h,b ) => h*b,
            Self::Square(Side1) => (Side1 * Side1) as f32,
        }
    }
}

fn main(){
    let shape = Shape::Circle(10.0);
    let result = shape.area();
    println!("Result {} ", result);
}   