//ch3 is about Common Programming Concepts in rust.

use super::exer;
use std::io;
fn shadowing_x() {
    let x: u8 = 5;
    let x = x + 1;
    let z = '🤣';
    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}"); //12
    }
    println!("The value of x is {x} and z is {z}"); //6, 🤣.
}

fn tup() {
    //THis is a tuple and they can not grow and srink;
    let tup = (12.9, 57, 'A', "Ajay");
    //we can destructure a tuple in rust like this;
    let (f, i, c, s) = tup;
    println!(
        "THis is a tuple: {:?} & these are it's values, f: {f}, i: {i}, c: {c}, s: {s}",
        tup
    );
    let i = tup.1 + 7;
    let tup2 = ("26", 2);
    println!("THis is modified tuo value (i = 57): {i}");
    let x: i32 = tup2
        .0
        .trim()
        .parse()
        .expect("Please make sure that the string can be converted on a numner.");
    println!("THis values is converted from str(which is a part of tuple) to number: {x}.")
}

//Unsure whether to use an array or a vector,
//chances are you should use a vector.
//Arrays are more useful when you know the number
//of elements will not need to change.
//For example, if you were using the names of the
//month in a program, you would probably use an array
//rather than a vector because you know it will always
//contain 12 elements.

fn arr() {
    let arr1 = [12, 57, 75, 69];
    let arr2 = [3; 3]; //same as [3, 3, 3];
    let arr1_2 = arr1[2]; //75
    println!("These are array examples: {:?}, {}\n", arr2, arr1_2);
}

//This code can panic;
fn arr_panic() {
    let a: [u8; 6] = [1, 3, 5, 7, 9, 2];
    println!("Please Enter the index of the array.");
    println!("If you will enter value more than 6, the thread will panic.\n");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Please input a valid number.");
    let index: usize = index
        .trim()
        .parse()
        .expect("PLese enter a valid integer number between 0-6.");
    let element = a[index];
    println!(
        "Element {element} is at index {index} in the array {:?}\n",
        a
    )
}

//Functions in rust;
//values passed inside functions brackets fn(x, y) are known as
//'parameters' and the concrete values entered when calling the functions
//are known as arguments, ex: fn (2, 5);

fn expression() {
    let x = {
        println!("Please enter a value < 249, this will get increaseb by 7: ->");
        let mut z = String::new();
        io::stdin()
            .read_line(&mut z)
            .expect("Enter a valid number.");
        let z: u8 = z.trim().parse::<u8>().expect("Not a valid number") + 7;
        z
    };
    println!("\nThe value of x is {:?}\n", x)
}

//Control flows in rust;
//loop in rust
fn loop_() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

//Loop Labels to Disambiguate Between Multiple Loops

fn loop_label() {
    println!("\n\nnested loops: ");
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
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
    println!("End Count = {count}")
}

pub fn main() {
    shadowing_x();
    tup();
    arr();
    arr_panic();
    expression();
    loop_();
    loop_label();
    println!("----------ch3-Exercise-----------");
    exer::main();
}
