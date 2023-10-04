use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let blue = scores.get("Blue");
    let yellow = scores.get("Yellow");
    println!("{:?}", blue.copied().unwrap_or(0));
    println!("{:?}", yellow.copied().unwrap_or(0));

    scores.entry(String::from("Green")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
