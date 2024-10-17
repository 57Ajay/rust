//This chapter covers ownerships in rust;
// It will be wise to read this chapter on the book
// again and again

fn own(owned_str: String) {
    let s = "hello";
    let y;
    {
        y = "World";
    }
    println!("This is value of s: {s} and this is of y: {y}");
    let mut st = String::from("Ajay");
    st.push_str(" Upadhyay");
    println!("My name: {st}, and this is owned value: {owned_str}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn main() {
    let s = String::from("Owned str.");
    own(s);
    let s1 = String::from("AJay Upadhyay");
    println!("{:?}", calculate_length(&s1));
}
