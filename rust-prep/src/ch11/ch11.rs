// there are more stuff in test and you should probably see the rust book
// for more features like ignoring some test (#[ignore]) amd much more features
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn incr(val: &mut i32) -> i32 {
    *val += 1;
    *val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn my_test() -> Result<(), &'static str> {
        let mut x = 1;
        incr(&mut x);
        incr(&mut x);
        if x == 3 {
            Ok(())
        } else {
            Err("SOMETHING WENT WRONG")
        }
    }
}

pub fn main() {
    println!("Hello from chapter 11");
}
