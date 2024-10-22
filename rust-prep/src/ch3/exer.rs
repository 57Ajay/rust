use std::{
    fs::{self, File},
    io::{self, BufRead},
    path::Path,
};

fn nth_fib(mut n: u8) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut i = 0;
    let mut j = 1;
    let mut next: i32;
    loop {
        next = i + j;
        i = j;
        j = next;
        n -= 1;
        if n == 1 {
            break;
        }
    }
    next
}

fn days_of_chrismas() -> io::Result<()> {
    let path = Path::new("resources/twelve_days_of_christmas.txt");

    let file = fs::File::open(&path)?;
    //let mut s = "Four Calling Birds".to_string();
    let reader = io::BufReader::new(file);
    //println!("found the line: {:?}", reader.read_line(&mut s));
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn find_in_file(path: &str, keyword: &str) -> io::Result<Vec<(usize, String)>> {
    let file = File::open(path)?;
    let render = io::BufReader::new(file);
    let mut results = Vec::new();

    for (line_number, line) in render.lines().enumerate() {
        let line = line?;
        if line.contains(keyword) {
            results.push((line_number + 1, line));
        }
    }
    Ok(results)
}
#[allow(dead_code)]
fn call_finder() -> io::Result<()> {
    let path = "resources/twelve_days_of_christmas.txt";
    let keyword = "  And a Partridge in a Pear Tree".trim();
    match find_in_file(path, keyword) {
        Ok(results) => {
            if results.is_empty() {
                println!("No occurrences of '{}' found.", keyword);
            } else {
                for (line_number, line) in results {
                    println!("Found on line {}: {}", line_number, line);
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }

    Ok(())
}

fn call_finder_with_params(path: &str, keyword: &str) -> io::Result<()> {
    match find_in_file(path, keyword) {
        Ok(results) => {
            if results.is_empty() {
                println!("No occurrences of '{}' found.", keyword);
            } else {
                for (line_number, line) in results {
                    println!("Found on line {}: {}", line_number, line);
                }
            }
        }
        Err(e) => println!("Error reading file: {}", e),
    }

    Ok(())
}

pub fn main() {
    let mut n = String::new();
    println!("Please input a integer n to find nth fib number: ");
    io::stdin().read_line(&mut n).expect("Input a valid number");
    let n = n.trim().parse().expect("Not a valid Integer");
    println!("The {n}-th fib number is: {:?}", nth_fib(n));
    println!("\n-----------------Lines of the song 12 days of chrismas---------------\n");
    days_of_chrismas().expect("Failed to read lines of the song.");
    println!("\n-------Word and sentence finder in a file------- \n");
    call_finder_with_params(
        "resources/twelve_days_of_christmas.txt",
        "Partridge in a Pear Tree",
    )
    .expect("Failed to read the file, make sure that the oath is correct.");
}
