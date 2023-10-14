use std::fs::File;

fn isPrime(num: i32, rec: i32) -> bool {
    let mut ret = false;
    if num == 1 || rec == 1 {
        return true;
    } else if num % rec == 0 {
        return false;
    } else {
        ret = isPrime(num, rec - 1);
    }

    ret
}

fn isPrimeIter(num: i32) -> bool {
    let mut i = num / 2;
    while (i > 1) {
        if num % i == 0 {
            return false;
        }
        i -= 1;
    }
    return true;
}

fn main() {
    println!("64 -> {}", isPrime(64, 63));
    println!("73 -> {}", isPrime(73, 72));
    println!("64 -> {}", isPrimeIter(64));
    println!("73 -> {}", isPrimeIter(73));
}
