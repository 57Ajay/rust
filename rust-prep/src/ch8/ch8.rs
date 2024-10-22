//THis is all about common collections in std module;
use super::exer;
use std::collections::HashMap;
//1. Vectors
fn vec() {
    let arr = vec![21.0, 5.7, 56.98, 9.0];
    let index = arr.len() - 1;
    let val = arr.get(index);
    match val {
        Some(i) => println!("The val at index {index} is : {i}"),
        None => println!("No value founded at index {index}"),
    }
}
#[derive(Debug)]
#[allow(dead_code)]
enum Scell {
    Int(i32),
    Float(f32),
    Text(String),
}
fn vec_scell() {
    let a = vec![
        Scell::Int(12),
        Scell::Float(5.7),
        Scell::Text(String::from("AJay Upadhyay")),
    ];
    let name = &a[2];
    match name {
        Scell::Text(text) => println!("The name is {}", text),
        Scell::Int(_) => println!("This is an Int variant."),
        Scell::Float(_) => println!("This is a Float variant."),
    }
    println!("This is vector a: {:#?}", a);
    let mut v = Vec::new();
    v.push(12);
    v.push(21);
    while let Some(i) = v.pop() {
        println!("{i}")
    }
}

//2. Strings
fn string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('s');
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    let x = s.replace("ic", "xz");
    println!("s: {s}, x: {x}");
    let r = s.contains("-tac");
    println!("r: {r}")
}

//3. HashMaps
fn hashmap() {
    let mut scores = HashMap::new();
    scores.insert("blue", 150);
    scores.insert("red", 125);
    scores.insert("green", 157);
    scores.entry("green").or_insert(175);
    scores.entry("black").or_insert(137);
    let score = scores.get("green").copied().unwrap_or(0);
    println!("score green: {score}, scores: {scores:#?}");

    let mut word_map = HashMap::new();
    let text = "Hello world! this is AJay from Indore.";
    for i in text.split_whitespace() {
        let count = word_map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("\n words counters: {word_map:#?}");
}

pub fn main() {
    vec();
    vec_scell();
    string();
    hashmap();
    println!("\n-----------------ch8 exercise--------------------\n");
    exer::main();
}
