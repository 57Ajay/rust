// ALL these stuffs are directly from rust book

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic ->
fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// traits -> Traits: Defining Shared Behavior
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[allow(dead_code)]
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn trait_main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// =================================================LIfetimes =================================================;

fn largest_str<'a, 'b, 'c>(s1: &'a str, s2: &'b str) -> &'c str
// can be done woth only one
// lifetime but just checking
// with multiple one
where
    'a: 'c,
    'b: 'c,
{
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn life() {
    let s1 = String::from("qwert");
    let s2 = "str2";
    println!("{}", largest_str(&s1, s2));
}

pub fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list); // largest_i32
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list); // largest_char
    println!("The largest char is {result}");

    trait_main();

    let pair = Pair::new(57, 12);
    pair.cmp_display();
    life();
}
