use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(error) => Err(error)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => return Ok(username),
        Err(e) => return Err(e)
    };
}

fn read_username_from_file_better() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_best() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // Just be gigachad
    fs::read_to_string("hello.txt");
}