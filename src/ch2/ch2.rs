use rand::Rng;
use std::io;

fn guess_num_print() {
    println!("Guess the number!");
    let secret_num: u8 = rand::thread_rng().gen_range(1..=100);
    let mut number_of_guesses: u8 = 0;
    loop {
        number_of_guesses += 1;
        println!("Guess {number_of_guesses}, There are total 10 guesses only.");
        if number_of_guesses > 10 {
            println!("Sorry you lost, please try again later.");
            break;
        }
        let mut guess = String::new();
        println!("please input the guessed number.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!, in {} guesses.", number_of_guesses);
                break;
            }
        }
    }
}

pub fn main() {
    guess_num_print();
}
