use std::fs::File;
use std::io::ErrorKind;

fn main () {
    let greeting_file_result = File::open("hello.txt");

    // METHOD 1
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(f) => f,
    //             Err(e) => panic!("Could not create file - {e}")
    //         },
    //         other => panic!("Could not open file {other}")
    //     }
    // };

    // METHOD 2
    // let greeting_file_result = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // })

        
    
}

