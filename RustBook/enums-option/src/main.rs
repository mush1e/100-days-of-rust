fn main() {
    let some_number = Some(5);
    let some_char   = Some('e');

    let absent_number : Option<i32> = None;

    let x = 5;
    let y : Option<i32> = None;

    println!("{}", x+y.unwrap_or(5));
    
}
