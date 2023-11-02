fn checkPrimeRecursive(num : i32, div : i32) -> bool {
    if div == 1 || num == 1 {
        return true;
    } else if num % div == 0{
        return false;
    } else {
        return checkPrimeRecursive(num, div-1);
    }
}
fn main () {
    let x = [64, 32, 7, 5, 9];
    println!("Welcome to the dumbass program");

    for num in x {
        println!("{}", checkPrimeRecursive(num, num-1));
    }
}