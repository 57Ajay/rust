// Smart Pointers -> Box<T>, Rc<T>, Arc<T>, (Ref<T>, refMut<T>) accessed thorough -> RefCell<T>
// 1. Box<T> pointer
// USES:
//a. When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
//b. When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
//c. When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

#[allow(dead_code)]
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn box_ptr_main() {
    use List::Cons;
    let cons_list = Cons::<&str>(
        "a",
        Box::new(Cons("j", Box::new(Cons("a", Box::new(List::Nil))))),
    );
    println!("{:?}", cons_list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // since we have deref impl there is no need of doing hello(&*m)
}

// custom drop method impementation

struct CustomPtr<'a> {
    data: &'a str,
}

impl Drop for CustomPtr<'_> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn main() {
    box_ptr_main();
    let _c = CustomPtr { data: "data" };
    println!("done with CustomPtr");
    rc_main();
    ref_cell_main();
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// Rc<T> pointer

#[allow(dead_code)]
enum RcList<T> {
    Cons(T, Rc<RcList<T>>),
    Nil,
}

fn rc_main() {
    use RcList::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a)); // Here this clone is not like reguler clone which creates a
                                     // shallow copy, bere it simply increases reference count,
                                     // for better understanding see this -> https://doc.rust-lang.org/book/ch15-04-rc.html#cloning-an-rct-increases-the-reference-count

    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// RefCell<T> and the Interior Mutability Pattern
// Mutating the value inside an immutable value is the interior mutability pattern.
fn ref_cell_main() {
    // this one is from the end so ignore it and move below
    // the function to understand this code;
    use RefcellList::{Cons, Nil};
    println!("\nstart of ref_cell_main\n");
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");

    // leak_main();
    leak_fix_main();
}

#[allow(dead_code)]
trait Messenger {
    fn send(&self, msg: &str);
}

#[allow(dead_code)]
struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

#[allow(dead_code)]
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum RefcellList {
    Cons(Rc<RefCell<i32>>, Rc<RefcellList>),
    Nil,
}

// Memory leak time

#[derive(Debug)]
enum RefLeak<T> {
    Cons(T, RefCell<Rc<RefLeak<T>>>),
    Nil,
}

#[allow(dead_code)]
impl<T> RefLeak<T> {
    fn tail(&self) -> Option<&RefCell<Rc<RefLeak<T>>>> {
        use RefLeak::{Cons, Nil};
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[allow(dead_code)]
fn leak_main() {
    use RefLeak::{Cons, Nil};
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a.tail());
}

//Prevention from memory leak

#[allow(dead_code)]
#[derive(Debug)]
struct Node<T> {
    value: T,
    children: RefCell<Vec<Rc<Node<T>>>>,
    parent: RefCell<Weak<Node<T>>>,
}

fn leak_fix_main() {
    println!("\nHello from leak_fix_main.\n");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
