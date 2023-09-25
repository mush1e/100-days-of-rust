fn main() {
    // let mut x = 5;
    // println!("the value of x is {x}");           Basic example of the mutability of let variables in rust
    // x = 6;
    // println!("the value of x is {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is {x}");
    }
    println!("the value of x in the outer scope is {x}");

    // take care of shadowing. think about it 
    // use your brain dont be stinky poopoo head pls

    //Rust types -> Scalar types, Compound types
        // Scalar types -> integers, floats, boolean, characters
        //      u/i8,16,32,64,128,size
        //      f32, f64
        //      true, false 
        // Compound types -> tuples and arrays     

}
