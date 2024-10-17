mod ch1 {
    pub mod ch1;
}
mod ch2 {
    pub mod ch2;
}
mod ch3 {
    pub mod ch3;
    pub mod exer;
}
mod ch4 {
    pub mod ch4;
}
fn main() {
    println!("---------ch1---------");
    ch1::ch1::main();
    println!("---------ch1---------\n");
    println!("---------ch2---------\n");
    ch2::ch2::main();
    println!("---------ch2---------\n");
    println!("---------ch3---------\n");
    ch3::ch3::main();
    println!("---------ch3---------\n");
    println!("---------ch4---------");
    ch4::ch4::main();
    println!("---------ch4---------\n");
}
