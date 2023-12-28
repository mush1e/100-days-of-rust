fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n-1) + fibonacci(n-2)
}

fn main() {
    let num = fibonacci(9);
    println!("{num}");
}
