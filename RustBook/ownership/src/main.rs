fn modify_string(s: &mut String) {
    s.push_str(" world!");
}

fn print_string(s: String) {
    println!("{}", s);
}

fn main() {
    let mut str = String::from("Hello ");
    modify_string(&mut str);
    println!("{}", str);

    let str = String::from("Hiyya ");
    print_string(str);
    println!("{}", str);
}

// Understanding basic borrowing and ownership
//  