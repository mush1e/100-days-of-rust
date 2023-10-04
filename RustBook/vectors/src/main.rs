#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    // empty vec
    let v: Vec<i32> = Vec::new();
    // more common 
    let v = vec![1, 2, 3];
    // new with alues being pushed
    let mut v : Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(9);

    let third: &mut i32 = &mut v[2];
    *third = 5;
    println!("The third element is {third}");

    let v = vec![1,2,3,4,5];
    let dne = v.get(100);

    println!("{:?}", dne);

    for e_ref in &v {
        println!("{}", *e_ref + 1);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("green")),
    ];

    for i in 0..row.len() {
        println!("{i} - element of row => {:?}", row[i]);
    }
}
 