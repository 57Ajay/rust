use std::time::Instant;

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
mod ch5 {
    pub mod ch5;
}
mod ch6 {
    pub mod ch6;
}
mod ch7 {
    pub mod ch7;
}
mod ch8 {
    pub mod ch8;
    pub mod exer;
}
mod ch9 {
    pub mod ch9;
}
mod ch10 {
    pub mod ch10;
}
mod ch11 {
    pub mod ch11;
}
mod ch12 {
    pub mod ch12;
}

#[allow(dead_code)]
fn main() {
    let now = Instant::now();
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
    println!("---------ch5---------");
    ch5::ch5::main();
    println!("---------ch5---------\n");
    println!("---------ch6---------");
    ch6::ch6::main();
    println!("---------ch6----------\n");
    println!("---------ch7----------");
    ch7::ch7::main();
    println!("---------ch7----------\n");
    println!("---------ch8----------");
    ch8::ch8::main();
    println!("---------ch8----------\n");
    println!("---------ch9----------");
    ch9::ch9::main();
    println!("---------ch9----------\n");
    println!("---------ch10----------");
    ch10::ch10::main();
    println!("---------ch10----------\n");
    println!("---------ch11----------");
    ch11::ch11::main();
    println!("---------ch11----------\n");
    println!("---------ch12----------\n");
    ch12::ch12::main();
    println!("---------ch12----------\n");

    let elapsed_time = now.elapsed();
    println!(
        "Running all these chapters took {} seconds.",
        elapsed_time.as_secs()
    );
}
