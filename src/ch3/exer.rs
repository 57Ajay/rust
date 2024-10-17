use std::{
    fs,
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

pub fn main() {
    let mut n = String::new();
    println!("Please input a integer n to find nth fib number: ");
    io::stdin().read_line(&mut n).expect("Input a valid number");
    let n = n.trim().parse().expect("Not a valid Integer");
    println!("The {n}-th fib number is: {:?}", nth_fib(n));
    println!("\n-----------------Lines of the song 12 days of chrismas---------------\n");
    println!(
        "------------------------------------------------------------\n{:?}",
        days_of_chrismas()
    )
}
