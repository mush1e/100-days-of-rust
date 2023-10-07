fn print_fruit() -> Option<String> {
    Some(String::from("Banana"))
}

fn main() {
    println!("{}", print_fruit().unwrap_or(String::from("Invalid fruit")));
}
