#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn area (rectangle : &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let arb_value = 15;
    let rectangle = Rectangle {
        width: dbg!(2 * arb_value),
        height: 50
    };

    println!(
        "the area of the rectangle is {} units", area(&rectangle)
    );

    dbg!(&rectangle);
}
