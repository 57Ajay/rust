// Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams
// see trpl directory cloned from -> https://github.com/rust-lang/book/blob/main/packages/trpl
use std::collections::HashMap;
use std::future::Future;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::Duration;
use trpl;

fn trpl_main() {
    trpl::run(async {
        // will block untill executed
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(5)).await;
        }
        drop(tx);

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}

pub fn main() {
    println!("-----------------------------------");
    trpl_main();
    let res = fetch_example();
    println!("{res}");
    println!("-----------------------------------");
    test_html_parser();
    println!("-----------------------------------");
    run_waker();
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
