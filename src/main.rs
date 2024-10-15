mod ch1 {
    pub mod ch1;
}
mod ch2 {
    pub mod ch2;
}
mod ch3 {
    pub mod ch3;
}
fn main() {
    println!("---------ch1---------");
    ch1::ch1::main();
    println!("---------ch1---------\n");
    println!("---------ch2---------");
    ch2::ch2::main();
    println!("---------ch2---------\n");
    println!("---------ch3---------");
    ch3::ch3::main();
    println!("---------ch3---------\n");
}
