#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


fn main() {
    println!("Hello, world!");
    println!("{:#?}", Structure(4));
    println!("{:#?}", Deep(Structure(7)));
} 


