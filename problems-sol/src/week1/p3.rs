fn time_conversion(s: &str) -> String {
    let len = s.len();
    let new_str = s[2..len - 2].to_string();
    let time_type = s[(len - 2)..len].to_string();
    let time_hrs = s[0..2].to_string();
    let mut time_hrs: u8 = time_hrs
        .trim()
        .parse()
        .expect("Not a valid string to convert into string.");
    if time_type == "PM" {
        if time_hrs < 12 {
            time_hrs += 12;
        }
        let time = time_hrs.to_string() + &new_str;
        time
    } else {
        if time_hrs == 12 {
            let mut x = String::from("00");
            x.push_str(&new_str);
            return x;
        } else {
            s[0..len - 2].to_string()
        }
    }
}

pub fn main() {
    println!("THis is week1 P3");
    println!("{}", time_conversion("12:05:45AM"));
}
