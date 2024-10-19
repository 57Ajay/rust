//Chapter 5 is all about structs,
//These are like objects in other languages.
#[derive(Debug)]
#[allow(dead_code)]

struct User {
    name: String,
    age: u8,
    active: bool,
    email: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Color(u8, u8, u8);

#[derive(Debug)]
#[allow(dead_code)]
struct Point(u8, u8, u8);

fn use_struct() -> User {
    let mut user1 = User {
        name: String::from("Ajay"),
        email: String::from("57ajay.u@gmail.com"),
        age: 22,
        active: true,
    };
    user1.name = String::from("Ajay Upadhyay");
    let user2 = User {
        name: String::from("Vasudev singh parihar"),
        ..user1
    };
    let mut green = Color(0, 250, 0);
    green.1 = 255;
    let origin = Point(5, 7, 2);
    println!("green: {:?} origin: {:?}", green, origin);
    user2 //since user1 is partially moved into user2, hence it can not
          //be used now;
}
#[derive(Debug)]
struct Rectangle {
    length: u32,
    bredth: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.bredth
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.bredth * self.length
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.bredth > rect.bredth && self.length > rect.length {
            true
        } else {
            false
        }
    }
}

pub fn main() {
    println!("{:#?}", use_struct());
    let rect1 = Rectangle {
        length: 12,
        bredth: 31,
    };
    dbg!(&rect1);
    let rect2 = Rectangle {
        length: 11,
        bredth: 30,
    };
    println!("area of rect1: {:?}", area(&rect1));
    println!("area of rect1 with method area: {:?}", rect1.area());
    println!("Can rect1 hold rect2? : {:?}", rect1.can_hold(&rect2));
}
