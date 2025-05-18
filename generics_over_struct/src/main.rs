struct Rect<T>{
    width : T,
    length : T
}



impl<T:std::ops::Mul<Output = T> + Copy> Rect<T>   {
    fn area(&self) -> T {
        return self.length * self.width;
    }
}
fn main() {
    println!("Hello, world!");
    let r1 = Rect{
        length : 20,
        width : 20,
    };
    println!("{}",r1.area());
}

