fn main() {
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
}
