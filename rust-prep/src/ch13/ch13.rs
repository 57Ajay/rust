//Functional Language Features: Iterators and Closures

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn ex1_main() {
    let store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
        ],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn iters_main() {
    let mut v = vec![1, 3, 5];
    let mut v_iter = v.iter_mut();

    for i in &mut v_iter {
        *i += 1;
    }
    println!("{:?}", v_iter);
    println!("{:?}", v);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2 = v1.iter().map(|x| x + 1).collect::<Vec<i32>>(); // the compiler here automatically
                                                             // derefs the x as *x + 1 as `+` is
                                                             // implemented for i32 type and here x
                                                             // is &i32 so dereferencing it gives
                                                             // us i32;
                                                             // so under the hood compiler is doing
                                                             // this: (|x| *x + 1)

    assert_eq!(v2, vec![2, 3, 4]);
    assert_eq!(v1, vec![1, 2, 3]);
}

pub fn main() {
    ex1_main();
    let close = |x| -> i32 { x + 1 };
    let z = close(69);
    println!("{}", z);

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value.clone());
        r.width
    });
    println!("{list:#?}");

    iters_main();
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s| s.size == shoe_size)
        .collect::<Vec<Shoe>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
