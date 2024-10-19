mod week1 {
    pub mod p1;
    pub mod p2;
    pub mod p3;
    pub mod p4;
}

use std::{
    collections::HashSet,
    io::{self},
    vec,
};

fn num_ratio(arr: &[i32]) {
    let len = arr.len() as f32;
    let mut seen = HashSet::new();
    for &i in arr {
        if !seen.contains(&i) {
            let mut counter = 0;
            for &j in arr {
                if i == j {
                    counter += 1;
                }
            }
            seen.insert(i);
            let ratio = counter as f32 / len;
            println!("Element: {}, Ratio: {:.6}", i, ratio);
        }
    }
}

fn call_num_ratio() {
    let mut arr = vec::Vec::new();
    let mut arr_size = String::new();
    println!("Enter the length of the array: ");
    io::stdin()
        .read_line(&mut arr_size)
        .expect("Enter a valid number");
    let arr_size: i32 = arr_size.trim().parse().expect("Number: P1");
    for i in 0..arr_size {
        // If we keep arr_nums out of the loop, that will cause panic after 2nd
        // input because it wont get cleared strings aren't automatically cleared
        // after each iteration, the new user input is appended to the previous
        // input on each loop iteration. This means that on the second
        // iteration the string includes both the first and second inputs
        // combined,which cannot be parsed into an integer (arr_nums);
        println!("Enter number {} of the array.", i + 1);
        let mut arr_nums: String = String::new(); // if we keep this out of the
        io::stdin()
            .read_line(&mut arr_nums)
            .expect("Enter a valid numbers");
        let arr_nums: i32 = arr_nums.trim().parse().expect("Number: P2");
        arr.push(arr_nums);
    }
    num_ratio(&arr);
}

fn new_line() {
    println!("\n\n");
}

fn main() {
    week1::p1::main();
    new_line();
    week1::p2::main();
    new_line();
    week1::p3::main();
    new_line();
    week1::p4::main();
    new_line();
    call_num_ratio();
}
