fn first_word(s :&String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("Hello world!");
    let word = first_word(&s);
    println!("last index of the first word is {}", word);
    s.clear();
}
