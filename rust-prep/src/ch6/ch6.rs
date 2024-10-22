//This chapter is all about enums;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
#[allow(dead_code)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_type: &IpAddrKind) -> &IpAddrKind {
    ip_type
}
#[derive(Debug)]
#[allow(dead_code)]
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
#[allow(dead_code)]
enum Messages {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Messages {
    fn send(&self) -> String {
        match self {
            Messages::Write(message) => format!("Sending message... {}", message),
            _ => format!("Sending message... {:?}", &self),
        }
    }
}

//match Control flow Construct;
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// control flow with if let;
fn if_let() {
    let x = Some(String::from("Ajay"));
    if let Some(i) = x {
        println!("User's name: {:?}", i);
    } else {
        println!("Not reguler user");
    }
}

pub fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("V4: {:?} V6: {:?}", route(&four), route(&six));
    let ip_address = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    dbg!(ip_address);
    let home = IpAddrKind2::V6(String::from("::1"));
    println!("{:#?}", home);
    let message = Messages::Write(String::from("Where are you?"));
    println!("{:?}", message.send());
    println!(
        "Value of coin in cents: {:#?}",
        value_in_cents(Coin::Quarter)
    );
    let val = plus_one(Some(12));
    let result = match val {
        Some(v) => v,
        None => -1,
    };
    println!("{:?}", result);
    if_let();
}
