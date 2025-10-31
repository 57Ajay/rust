// fearless concurrency;

//1. Creating a New Thread with spawn

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn loop_with_limit(thread_name: &str, loop_limit: u8) {
    for i in 1..loop_limit {
        println!("hi number {i} from the {thread_name}!");
        thread::sleep(Duration::from_millis(1));
    }
}

fn threader() {
    // this one  is a spawned thread, so it will be shut down if main thread's execution is done
    // irrespective ov it's execution is done or still left
    // SEE function thread_join(), like how it handles from this situation
    thread::spawn(|| {
        loop_with_limit("spawned thread", 10);
    });
    // main thread
    loop_with_limit("main thread", 5);
}

fn thread_join() {
    let handle = thread::spawn(|| {
        loop_with_limit("spawn_thread_join", 10);
    });
    // line 29
    loop_with_limit("main_thread_join", 5);

    handle.join().unwrap(); // if i add this b/w spawned thread(handle variable) and main thread
                            // (at line 29)
                            // the main thread execution will start after the spawned thread is
                            // done;
}

fn move_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // if we remove move it will cause error, why?
        // see -> https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}

// Using Message Passing to Transfer Data Between Threads
fn channel_thread() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap(); // unwrap is a bad way to handle as in case of error it will
                                   // panic
            thread::sleep(Duration::from_millis(69));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(69));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

//Shared-State concurrency

fn mutex_main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn add_large_num(num: u64) -> u64 {
    let sum = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let chunk_size = (num / 4).max(1);

    for t in 0..4 {
        let start = t * chunk_size + 1;
        let end = if t == 3 { num } else { (t + 1) * chunk_size };
        let sum_clone = Arc::clone(&sum);

        let handle = thread::spawn(move || {
            let mut local_sum = 0;
            for i in start..=end {
                local_sum += i as u64;
            }

            *sum_clone.lock().unwrap() += local_sum;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let res = *sum.lock().unwrap();
    res
}

pub fn main() {
    // threader();
    // thread_join();
    // move_thread();
    // channel_thread();
    // mutex_main();
    println!("{}", add_large_num(1000));
}
