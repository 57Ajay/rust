//Chapter 1 is all about Rust installation and Cargo which is
//Rust’s build system and package manager.
//Important cargo commands:->
//We can create a project using -> 'cargo new'.
//We can build a project using -> 'cargo build'.
//We can build and run a project in one step using -> 'cargo run'.
//We can build a project without producing a binary to check for errors using
//-> 'cargo check'.
//and to relese the project -> 'cargo build --release'

//Ignore this to_be_used_by_ch3 fn untill ch3;
pub fn to_be_used_by_ch3() {
    println!("I am being used in ch3")
}
pub static X: u8 = 3;
pub fn main() {
    println!("This is rust chapter 1");
}
