use std::fs::read;

struct rect{
    height: u32,
    width : u32
}

impl rect{
    fn cal(&self) -> u32{
        return self.height * self.width
    }
    fn permimeter(&self) -> u32{
        return  2*(self.height * self.width );
    }
}

fn main() {
    println!("will be doing the implementing the struct in the rust! ohk here only!");
    let rect = rect{
        height : 10,
        width : 10,
    };
    println!("The area of the rectangle is {}",rect.cal()); 

    println!("The perimeter of the reactangle is {}",rect.permimeter());
}

