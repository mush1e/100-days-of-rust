fn print_string(str: String) {
    println!("{str}");
}

fn gives_ownership() -> String {
    String::from("Hello DumDum")
}

fn takes_and_gives_ownership(str_: String) -> String {
    str_
}

fn borrowing_poggers(str: &String) -> usize {
    str.len()
}

fn borrowing_and_mutating(s: &mut String) {
    s.push_str(" Siddiqui");
}

fn main() {
    let s1 = String::from("Hello world");
    let s2 = gives_ownership();
    let s3 = String::from("Mustafa");
    let mut s3 = takes_and_gives_ownership(s3);
    let l3 = borrowing_poggers(&s3);
    borrowing_and_mutating(&mut s3);

    print_string(s1);
    println!("{s2} {s3} {l3}");
    let hello_world = String::from("Hello World");
    let hello = &hello_world[..5];
    let world = &hello_world[6..];

    println!("{} {}", hello, world);
}
