fn main() {
    // let number = 3;

    // if number > 3 {   
    //     println!("> 3");
    // }
    // else {
    //     println!("3");
    // }

    // let condition = true;

    // let num = if condition {5} else {6};
    // println!("{num}");

    // loop {
    //     println!("im printing");
    // }

    // labelled_loops();
    // let mut counter = 0;

    // let result = loop {
    //     counter += 10;
    //     if counter == 10 {
    //         break counter*2;
    //     }
    // };
    // println!("{result}");

    condtional_while();
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn labelled_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("{count}");
        let mut remaining = 10;
        loop {
            println!("{remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn condtional_while() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

}
