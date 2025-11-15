use std::boxed::Box;

pub fn main() {
    let mut avc = AveragedCollection::new();
    avc.add(69);
    avc.add(138);
    avc.add(34);
    avc.add(35);
    avc.add(21);
    avc.remove();
    println!("{}", avc.average());

    println!(
        "\n||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||\n"
    );

    gui_main();
    blog_main();
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl<'a> AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: Vec::<i32>::new(),
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // here we will have access to average as we are in same crate but
    // if it was a lib used by some other crate then it would have been
    // private, hence inaccessible
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// -------------GUI TIME-------------------------

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[allow(dead_code)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Button {
    fn new(w: u32, h: u32, label: String) -> Self {
        Self {
            width: w,
            height: h,
            label,
        }
    }
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing button");
    }
}

#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl SelectBox {
    fn new(w: u32, h: u32, options: Vec<String>) -> Self {
        Self {
            width: w,
            height: h,
            options,
        }
    }
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing SelectBox");
    }
}

fn gui_main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox::new(
                6,
                6,
                vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            )),
            Box::new(Button::new(6, 9, String::from("OK"))),
        ],
    };
    screen.run();
}

// Blog post TIME-------------------------

fn blog_main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    post.approve(); // twice approval needed, see below
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("all asserted");
    post.reject();
}

struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    pub fn new() -> Post {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        let cont = self.state.as_ref().unwrap();
        cont.content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn can_add_text(&self) -> bool {
        false
    }
}
struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn can_add_text(&self) -> bool {
        true
    }
}

struct PendingReview {
    approvals: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals == 1 {
            Box::new(Published {})
        } else {
            Box::new(PendingReview {
                approvals: self.approvals + 1,
            })
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
