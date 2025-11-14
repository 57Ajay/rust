// Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
// see trpl directory cloned from -> https://github.com/rust-lang/book/blob/main/packages/trpl
use std::collections::HashMap;
use std::future::Future;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::pin::{pin, Pin};
use std::process::Output;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;
use std::time::{Duration, Instant};
use trpl::{self, Either};

async fn trpl_main() {
    // trpl::run(async {
    //     // will block untill executed
    //     trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });
    //
    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    // });

    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel();
    //
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("future"),
    //     ];
    //
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         trpl::sleep(Duration::from_millis(1000)).await;
    //     }
    //     drop(tx);
    //
    //     while let Some(value) = rx.recv().await {
    //         println!("received '{value}'");
    //     }
    // });
    //
    // ok let's make above code concurrent

    let (tx, mut rx) = trpl::channel();
    let tx_clone = tx.clone();

    let f1 = async move {
        // we can simply remove this move after async but then we will have to
        // manually drop tx using drop(tx).
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for (i, val) in vals.into_iter().enumerate() {
            tx.send(val).unwrap();
            if i == 2 {
                trpl::sleep(Duration::from_millis(10)).await;
            } else {
                trpl::sleep(Duration::from_millis(1000)).await;
            }
        }
        // drop(tx); // if move after async is removed this line needs to be uncommented
    };

    let f2 = async {
        while let Some(val) = rx.recv().await {
            println!("from conc one -> {val}");
        }
    };

    let f3 = async {
        tx_clone.send("This one is from me".to_string()).unwrap();
        drop(tx_clone);
    };

    // trpl::join3(f1, f2, f3).await;
    // trpl::join!(f1, f2, f3); // better solution as we can pass any number of futures
    // or what we can do is push all the futures in a vector
    // and then use join_all function
    let mut futures = Vec::<Pin<Box<dyn Future<Output = ()>>>>::new(); // if we won't use
                                                                       // Pin it won't compile
                                                                       // why? check by removing
                                                                       // Pin
    futures.push(Box::pin(f1));
    futures.push(Box::pin(f2));
    futures.push(Box::pin(f3));

    trpl::join_all(futures).await;

    // --------------------------------------------------
    // this monstrocity can be avoided if I do this ->   |
    //  let tx1_fut = pin!(async move {                  |
    // --snip--                                          |
    // });                                               |
    //                                                   |
    // let rx_fut = pin!(async {                         |
    //     // --snip--                                   |
    // });                                               |
    //                                                   |
    // let tx_fut = pin!(async move {                    |
    //     // --snip--                                   |
    // });                                               |
    //                                                   |
    // let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = |
    //     vec![tx1_fut, rx_fut, tx_fut];                    |
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn race_main() {
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        slow("a", 10);
        slow("a", 20);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        slow("b", 10);
        slow("b", 15);
        slow("b", 350);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}

async fn bench() {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::sleep(one_ns).await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!(
        "'sleep' version finished after {} seconds.",
        time.as_secs_f32()
    );

    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::yield_now().await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!(
        "'yield' version finished after {} seconds.",
        time.as_secs_f32()
    );
}

async fn timeout<F: Future>(fut: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(fut, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

async fn test_timeout() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn trpl_join() {
    let fut1 = async {
        for i in 1..10 {
            println!("hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut1, fut2).await;
}

pub fn main() {
    // println!("-----------------------------------");
    // trpl::run(async { trpl_main().await });
    // let res = fetch_example();
    // println!("{res}");
    // println!("-----------------------------------");
    // test_html_parser();
    // println!("-----------------------------------");
    // run_waker();
    // println!("-----------------------------------");
    // trpl::run(async {
    //     trpl_join().await;
    // })
    println!("-----------------------------------");
    // trpl::run(async { race_main().await });
    // trpl::run(async { bench().await });
    stream_main();
    println!("-----------------------------------");
}

fn fetch_example() -> String {
    let host = "example.com";
    let port = 80;

    let mut stream = TcpStream::connect((host, port)).expect("Failed to connect");

    let request = format!(
        "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        host
    );

    stream
        .write_all(request.as_bytes())
        .expect("Failed to send request");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("Failed to read response");

    parse_html_basic(&response)
}

pub fn parse_html_basic(html: &str) -> String {
    let mut output = String::new();

    let html = html.to_lowercase();

    let extract_tag = |tag: &str| -> Vec<String> {
        let open = format!("<{}>", tag);
        let close = format!("</{}>", tag);
        let mut results = Vec::new();
        let mut start = 0;

        while let Some(open_idx) = html[start..].find(&open) {
            let start_idx = start + open_idx + open.len();
            if let Some(end_idx) = html[start_idx..].find(&close) {
                let end_abs = start_idx + end_idx;
                let content = html[start_idx..end_abs].trim();
                if !content.is_empty() {
                    results.push(content.to_string());
                }
                start = end_abs + close.len();
            } else {
                break;
            }
        }

        results
    };

    let tags = ["title", "h1", "h2", "h3", "p", "body", "div"];
    let mut parsed: HashMap<&str, Vec<String>> = HashMap::new();

    for tag in tags.iter() {
        parsed.insert(tag, extract_tag(tag));
    }

    if let Some(titles) = parsed.get("title") {
        for t in titles {
            output.push_str(&format!("Title: {}\n", t));
        }
    }

    for h in ["h1", "h2", "h3"] {
        if let Some(headers) = parsed.get(h) {
            for text in headers {
                output.push_str(&format!("Header: {}\n", text));
            }
        }
    }

    if let Some(paras) = parsed.get("p") {
        for p in paras {
            output.push_str(&format!("Paragraph: {}\n", p));
        }
    }

    if let Some(body) = parsed.get("body") {
        if !body.is_empty() {
            output.push_str(&format!("Body snippet: {:.100}...\n", body[0]));
        }
    }

    output
}

fn test_html_parser() {
    let html = r#"
        <html>
            <head><title>Example Domain</title></head>
            <body>
                <h1>Example Domain</h1>
                <p>This domain is for use in illustrative examples in documents.</p>
                <h2>Subheading</h2>
                <p>More text here.</p>
            </body>
        </html>
    "#;

    let parsed = parse_html_basic(html);
    println!("{}", parsed);
}

// A little experiment with future trait

struct MyFuture {
    count: u8,
}

impl Future for MyFuture {
    type Output = &'static str;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            self.count += 1;
            Poll::Pending
        } else {
            println!("Second poll — ready now!");
            Poll::Ready("done!")
        }
    }
}

fn dummy_waker() -> Waker {
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}
    fn dummy_raw_waker() -> RawWaker {
        RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
        )
    }
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn run_waker() {
    let mut my_future = MyFuture { count: 0 };
    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);
    let mut pinned = unsafe { Pin::new_unchecked(&mut my_future) };

    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Pending => println!("Not ready, polling again..."),
            Poll::Ready(val) => {
                println!("Got result: {}", val);
                break;
            }
        }
    }
}

// streaming time

use trpl::{ReceiverStream, Stream, StreamExt};

fn stream_main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
    trpl::run(async {
        let mut messages = pin!(get_messages().timeout(Duration::from_millis(200)));

        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            };
        }
    });

    ReceiverStream::new(rx)
}
