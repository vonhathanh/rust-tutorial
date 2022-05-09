fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size:u32) -> Rectangle {
        Rectangle{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle{
        width: 30,
        height: 31
    };

    println!("area of rect is {}", rect.area());
}
