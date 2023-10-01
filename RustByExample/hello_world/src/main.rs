fn main() {
    // Normal arguments
    println!("{} days!", 31);

    // Positional arguments, kinda pog
    println!("{0}. this is {1}. {1}, this is {0}", "Foo", "Bar");

    // Named arguments wot da hell 
    println!("my name is {name} and I like to write code in {language}", name="Mustafa", language="Rust");

    // Different formatting can be invoked by specifying the format character -> Intense pogging 
    // after a `:`.
    println!("Base 10:               {}",   69420); 
    println!("Base 2 (binary):       {:b}", 69420); 
    println!("Base 8 (octal):        {:o}", 69420); 
    println!("Base 16 (hexadecimal): {:x}", 69420); 
    println!("Base 16 (hexadecimal): {:X}", 69420); 

    // some cute formatting
    println!("{:>5}", 2); 
    println!("{:<5}", 3);

    println!("{number:0>width$}", number = 69, width = 15);

}
