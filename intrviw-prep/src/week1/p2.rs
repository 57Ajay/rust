use std::vec;

fn min_max_sum(arr: &[i32]) {
    let len = arr.len();
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    let total_sum: i64 = sorted_arr.iter().map(|&x| x as i64).sum();
    let max = total_sum - (sorted_arr[0] as i64);
    let min = total_sum - (sorted_arr[len - 1] as i64);
    println!("{} {}", min, max);
}

//just a little extra.
fn sort_arr(arr: &mut Vec<i32>) -> Vec<i32> {
    let len = arr.len();

    for _ in 0..len {
        for j in 0..len - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    arr.to_vec()
}

pub fn main() {
    println!("This is week1 P2");

    // array in rust can only ne indexed by usize not i32 or similer;
    let arr: Vec<i32> = vec![793810624, 895642170, 685903712, 623789054, 468592370];
    min_max_sum(&arr);
    let mut arr_ = vec![9, 12, 5, 7, 8, 0];
    println!("{:?}", sort_arr(&mut arr_));
}
