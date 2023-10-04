

fn main() {
    let mut s = String::new();
    let data = "intial content";
    // &str to String
    let s = data.to_string();
    let s = "initial content".to_string();
    let mut s = String::from("foo");
    
    // push_str and push
    s.push_str("bar");
    s.push('!');
    println!("{}", s);

    // using the + operator to concatenate shit
    // also deref coercion??
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let s3 = s1.clone() + &s2;

    println!("{} {} {}", s1, s2, s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
}
