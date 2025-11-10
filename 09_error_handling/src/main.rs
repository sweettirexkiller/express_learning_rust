use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("Crash and burn");

    // RESULT ENUM


    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("There was a problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Tried to create file but there was a problem: {:?}", error)
    //         })
    //     } else {
    //         panic!("There was a problem opening the file: {:?}", error)
    //     }
    // });

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // error propagation
    let f = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    // f.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("hello.txt")
}
