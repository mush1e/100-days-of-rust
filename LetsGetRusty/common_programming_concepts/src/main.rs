use std::io;

const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

// look function declaration (rust doesnt care where you declare it lexically)
// trailing return type (kinda like C++)

fn another_function(x: u32) -> u32{

    let x = {
        x * 2
    };

    println!("Function has been called :D --{x}");
    x
}

fn main() {

    // variable declaration stuff
    let mut x = 6;
    println!("{}", x);
    x = 5;
    println!("{}", x);
    println!("{}", THREE_HOURS_IN_SECONDS);


    let x = x + 1;
    {
        let x = x * 2;
        println!("{}", x);
    }
    println!("{}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);


    // messing around with tuples
    let tup = (10, 'z', 52);
    let (x, y, z) = tup;
    println!("{:?}", tup);
    println!("{:?}", tup.1);
    println!("{}", x);


    // Arrays pretty sick
    let a = [1,2,3,4,5,6];
    let b: [u8; 5] = [1,2,3,4,5];
    let b = [3; 5];
    println!("{:?}\n{:?}", b, a); 

    let mut buf = String::new();
    // array indexing
    io::stdin().read_line(&mut buf).expect("Unable to read line");

    let buf : usize = buf.trim().parse().expect("Not a number!");

    let element = a[buf];
    println!("the value of the element at index {buf} is {element}");
    
    //invoking function to do stuff
    println!("{}",another_function(5));

    let number = if true {5} else {10};

    println!("The value of number is: {number}");

    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count*2;
        }
    };

    println!("The result of the loop is {result}");

}