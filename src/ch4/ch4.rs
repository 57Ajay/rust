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

fn understand_borrowing() {
    let mut s = "Ajay";
    let r = &mut s;
    //let r1 = &s; // no problem, (will need 's' as not mutable)
    //    let r2 = &s; // no problem
    //    let r3 = &mut s; // BIG PROBLEM

    //let t = &mut s;  This is not possible as only one mutable reference is
    //allowed at once;

    println!("{}", r);
    let t = &mut s; // now it's possible as 1st mutable reference
                    // has already been used;
    println!("{}", t);
}

//understanding Dangling references

fn dangel() -> String {
    //&String not allowed
    let s = String::from("Ajay");
    s //&s not allowed
}

fn first_word(s: &str) -> &str {
    println!("From first word function; \n");
    let bytes = s.as_bytes();
    for (i, &j) in bytes.iter().enumerate() {
        if j == b' ' {
            println!("From inside for loop: ");
            return &s[0..i];
        }
    }
    println!("From outside the for loop");
    s
}

pub fn main() {
    let s = String::from("Owned str.");
    own(s);
    let s1 = String::from("AJay Upadhyay");
    println!("{:?}", calculate_length(&s1));
    understand_borrowing();
    println!("{:?}", dangel());
    println!("{:?}", first_word(&s1));

    let a = [2, 4, 6, 78, 9];
    let slice = &a[2..5];
    println!("Slice of a: {:?}", slice);
}
