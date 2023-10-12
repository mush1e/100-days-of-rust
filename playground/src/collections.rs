use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    // VECTORS
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 3];
    let v3 = v2.get(20);
    println!("{:?}", v3);
    match v.get(20) {
        Some(i) => println!("{}", i),
        _ => println!("None"),
    };

    // Strings
    let s1 = String::new();
    let s2 = "hello world"; // String slice
    let s3 = s2.to_string();
    let s4 = String::from(s2);

    for i in s3.graphemes(true) {
        println!("{i}");
    }

    // HashMaps <k, v>
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 50);
    scores.insert(String::from("Blue"), 30);

    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Green")).or_insert(30);

    println!("{:?}", scores.get("Green"));

    for (k, v) in &scores {
        println!("{} : {}", k, v)
    }
}
