fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_break = 0;
    let mut max_break = 0;
    let mut break_arr: Vec<i32> = vec![0, 0];

    for &i in scores {
        if i < min_score {
            min_break += 1;
            break_arr[1] = min_break;
            min_score = i;
        } else if i > max_score {
            max_break += 1;
            break_arr[0] = max_break;
            max_score = i;
        } else {
            min_score = min_score;
            max_score = max_score;
        }
    }
    break_arr
}

pub fn main() {
    println!("This is week1 P4");
    let scores = [12, 24, 10, 24];
    println!("{:?}", breaking_records(&scores))
}
