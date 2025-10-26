fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));

    let rand_limit = 100;
    println!(
        "the random number is: {}",
        add_one::get_random_val_upto(rand_limit)
    );

    println!("Hello, world! {num} plus two is {}!", add_two::add_two(num));
}
