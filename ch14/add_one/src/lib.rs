use rand::Rng;

pub fn get_random_val_upto(val: u8) -> u8 {
    let mut rng = rand::rng();
    rng.random_range(0..val)
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
    #[test]
    fn rand_test() {
        assert!(get_random_val_upto(69) < 69);
    }
}
