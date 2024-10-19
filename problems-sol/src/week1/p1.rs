fn plus_minus(arr: &[i32]) {
    let mut pc = 0;
    let mut nc = 0;
    let mut zc = 0;
    let len = arr.len() as f32;
    for &i in arr {
        if i > 0 {
            pc += 1;
        } else if i == 0 {
            zc += 1;
        } else {
            nc += 1;
        }
    }
    println!(
        "{:.6}\n{:.6}\n{:.6}",
        (pc as f32 / len),
        (nc as f32 / len),
        (zc as f32 / len)
    );
}

pub fn main() {
    println!("THis is week1 P1");
    plus_minus(&[2, 4, -6, 0, 0]);
}
