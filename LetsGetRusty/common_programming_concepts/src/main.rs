const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

fn main() {

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
}