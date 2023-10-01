#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // area method
    fn area(&self) -> u32 { 
        self.width * self.height
    }

    // random ass method -> showing methods can have the same names as struct members
    fn width(&self) -> bool {
        self.width > 0
    }

    // can_hold method
    fn can_hold (&self, other : &Self) -> bool {
        other.area() <= self.area()
    }

    fn square (size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 30,
        height: 20,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 50,
    };

    if rect2.width() {
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    }

    if rect3.width() {
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    let squar1 = Rectangle::square(3);
    println!("{:#?}", squar1);
    
}
