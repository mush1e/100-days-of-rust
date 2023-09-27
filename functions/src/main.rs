fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(10, 'h');
    scope_block();
    println!("{}", five());
}

fn another_function(x: u8) {
    println!("The value of x is {x}"); 
}

fn print_labeled_measurement(value: i32, unit_label: char) { 
    println!("The value of the measurement is {value} {unit_label}");
}

// must have type annotations for function parameters in Rust

//statements vs expressions -> statements do shit , expressions return shit

fn scope_block() {
    
    let x = {
        let x = 5;
        x + 1   // no semi colon? I still dont understand it but keep it in mind for now i.g
    };

    println!("{x}");
}

fn five() -> i32 {
    5
}
