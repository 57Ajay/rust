use std::io;
use std::process::Command;

pub fn main() {
    println!("This is ch7");
    // Chapter 7 is all about managing packages, crates, and modules in Rust
    let url = "https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html";

    println!("Do you want to view Chapter 7 now? (yes/no)");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    if input.trim().eq_ignore_ascii_case("yes") {
        if cfg!(target_os = "linux") {
            println!("Running on Linux");
            Command::new("xdg-open")
                .arg(url)
                .spawn()
                .expect("Failed to open browser on Linux");
        } else if cfg!(target_os = "macos") {
            println!("Running on macOS");
            Command::new("open")
                .arg(url)
                .spawn()
                .expect("Failed to open browser on macOS");
        } else if cfg!(target_os = "windows") {
            println!("Running on Windows");
            Command::new("cmd")
                .args(&["/C", "start", url])
                .spawn()
                .expect("Failed to open browser on Windows");
        } else {
            println!("Unsupported OS. Please visit: {}", url);
        }
    } else {
        println!("To learn more about Chapter 7, you can visit: {}", url);
    }
}
