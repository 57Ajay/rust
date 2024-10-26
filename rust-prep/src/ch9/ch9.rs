//THis chapter is all about handling errors in Rust;

//Panic! macro;

use core::panic;
use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

#[allow(dead_code)]
#[allow(unused)]
fn file_open() {
    //This is similer to what is in main fn but not using match{};
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//This uses ? (a sortcut) to propogate errors in Rust;
#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//This uses chaining to reduce code;
#[allow(dead_code)]
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// using ? in Option<T>;
#[allow(dead_code)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn main() {
    let file = File::open("test.rs");
    match file {
        Ok(x) => println!("THis is the response: {x:#?}"),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("test.rs") {
                Ok(fc) => println!("File created successfully: {fc:#?}"),
                Err(er) => println!("Error creating the file: {er:#?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    }
    //let _ = panic!("Crashed knowingly");
    println!("This is ch9");
}
