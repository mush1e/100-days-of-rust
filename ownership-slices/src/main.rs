fn first_word(s :&String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item==b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut index1 = s.len();
    let mut index2 = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if i < index1 {index1 = i} else {index2 = i; break;};
        }
    }
    if index1 > index2 {return &s[..];}
    &s[index1..index2]
}

fn gigachad_slices(s: &String) -> &str {
    let first_word = &s[..5];
    first_word
}

fn main() {
    let mut s = String::from("Hello world!");
    let word = first_word(&s);
    println!("last index of the first word is {}", word);
    s.clear();

    let mut s = String::from("My");
    let word = second_word(&s);
    println!("{}", word);
}
