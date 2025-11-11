// this is all just for understanding the async rust, because i want to know
// how a runtime executes the async funtion, I triesd to read tokio code a little
// but soon i realise i need to master some of rust's advanced topics, such as
// unsafe rust from https://doc.rust-lang.org/nomicon/, Generics, Traits, and Pointers
// and more below is the syllabus gemini suggested me to learn as i asked it to understand
// Tokio codebase:
//
//
// Here is the list of topics you need to be solid on, in order, before Tokio's source code will start to make sense.
// -----------------------------------------------------------------------------------------------------------------------------
// 1. Advanced Rust: Generics, Traits, and Pointers
//
// This is the bedrock. You need to be 100% fluent in how Rust structures code.
//
// Traits: Not just using them, but advanced use: dyn Trait, impl Trait (in arguments and return types),
// associated types, and trait bounds (<T: Send + Sync>).
//
// Smart Pointers: You must deeply understand Arc<T> and Box<T>.
// You also need to know why Arc<Mutex<T>> is a common pattern.
//
// Interior Mutability: Understand Cell<T> and RefCell<T>.
// This is for when you see &self on a method but it still mutates something inside.
//
// Marker Traits: The most important ones: Send and Sync. You must know what they mean, why
// they are "auto-traits," and what !Send or !Sync implies. Tokio's multi-threaded scheduler vs.
// the current-thread scheduler is built entirely around this.
//
// Closures: Specifically, the Fn, FnMut, and FnOnce traits. When you tokio::spawn,
// you're passing a closure, and the compiler needs to know what it "captures" and how.
//
// -----------------------------------------------------------------------------------------------------------------------------
// 2. The Guts of async / await
//
// This is the biggest hurdle. async / await is just syntactic sugar. You must understand what it's sugar for.
//
// The Future Trait: This is priority #1. You need to know that async fn just returns a struct that implements Future.
//
// This trait has one method: poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>.
//
// You must understand that the Poll enum has two variants: Poll::Ready(value) and Poll::Pending.
//
// Pin<T>: This is the hardest part. Tokio's Cargo.toml even pulls in a crate called pin-project-lite to help with this.
//
// Why it exists: async functions create state machines. These state machines are often
// "self-referential" (e.g., they hold a buffer and a pointer into that buffer).
//
// If that struct moved in memory (like Rust loves to do), those internal pointers would become invalid.
//
// Pin is a "promise" to the compiler that the data at a memory location will never move.
// It "pins" it. This makes it safe to use self-referential structs.
//
// Waker and Context: This is the magic.
//
// When your poll method returns Poll::Pending, how does the executor know when to poll it again?
//
// It doesn't! It's your future's job to "register" the Waker (which it gets from the Context)
// with whatever it's waiting on (like an I/O event from the driver).
//
// When the OS event arrives, the I/O driver calls .wake() on the Waker.
// This tells the executor, "Hey, that task is ready to be polled again."
//
// -----------------------------------------------------------------------------------------------------------------------------
// 3. Concurrency: Threads vs. Tasks
//
// std::sync vs. tokio::sync: You must know the difference.
//
// std::sync::Mutex blocks the thread. If you use this in an async task, you stall
// the entire scheduler thread, killing performance.
//
// tokio::sync::Mutex blocks the task. When you .await the lock, it returns Poll::Pending
// and gives the Waker to the mutex. The thread is now free to go run other tasks. This is the core of cooperative multitasking.
//
// Atomics and Memory Ordering: The schedulers and drivers use high-performance,
// lock-free code. This requires understanding AtomicUsize, AtomicBool, etc., and
// at least Ordering::SeqCst and Ordering::Acquire/Release. The presence of loom in
// the Tokio codebase is a dead giveaway that they are very serious about this.
//
// Thread Locals: You'll see thread_local! macros. Tokio uses these to store the
// "context" of the current runtime on a per-thread basis.
//
// -----------------------------------------------------------------------------------------------------------------------------
// 4. Low-Level unsafe and OS APIs
//
// unsafe Rust: Tokio is full of it. It has to be, to manage its own task memory
// and talk to the OS. You need to be comfortable seeing unsafe blocks and understanding
// why they are (hopefully) safe. This includes raw pointers (*const T, *mut T).
//
// mio (Metal I/O): Tokio's I/O driver is built on mio. mio is a thin, cross-platform wrapper
// around OS-level event systems: epoll (Linux), kqueue (macOS), and IOCP (Windows).
// You need to understand this "reactor" pattern.
//
// FFI (Foreign Function Interface): The libc and windows-sys dependencies show that Tokio calls
// C-style OS functions directly for things mio doesn't cover.
//
// -----------------------------------------------------------------------------------------------------------------------------
// 5. Metaprogramming (Macros)
//
// You're right, the macros are intense.
//
// Declarative Macros (macro_rules!): These are the "pattern-matching" ones. tokio::select!
// is a prime example. You need to be able to read their syntax.
//
// Procedural Macros: These are #[tokio::main]. This is code that writes code. #[tokio::main]
// literally takes your async fn main and rewrites it into a normal fn main that builds a Runtime and calls rt.block_on(...).
//
// Conditional Compilation (cfg): The tokio/Cargo.toml is full of "features." The code is littered
// with #[cfg(feature = "...")]. The tokio codebase is like 10 different crates combined, and the
// compiler just picks the pieces it needs. This is why it's so hard to read.
//
// -----------------------------------------------------------------------------------------------------------------------------
// Your Learning Plan
//
//
// Read "The Book" (The official Rust book), especially the chapters on traits, smart pointers, and concurrency.(almost done)
//
// Read "The Rustonomicon" (The official unsafe Rust book) to understand unsafe.
//
// The #1 Best Thing To Do: Write your own tiny async executor. Seriously. This is the way.
//
// Define your own Future.
//
// Create a Waker.
//
// Create a HashMap of tasks.
//
// Create a Vec of "tasks to run."
//
// When you poll a task and it's Pending, store its Waker.
//
// When a Waker is called, add its task ID to the "tasks to run" Vec.
//
// Loop.
//
// This single project will teach you 80% of what you're missing (Pin, Future, Waker, Context, Poll).
// There are great blog posts on this; search for "build your own async executor rust."
// -----------------------------------------------------------------------------------------------------------------------------
//

trait Speaker {
    fn speak(&self) -> &'static str;
}

struct Dog;
struct Cat;
impl Speaker for Dog {
    fn speak(&self) -> &'static str {
        "Woof!"
    }
}

impl Speaker for Cat {
    fn speak(&self) -> &'static str {
        "Meow!"
    }
}

// this is static dispatch
fn make_speak_static(speaker: &impl Speaker) {
    println!("(Static) It says: {}", speaker.speak())
}

// this one is dynamic dispatch
fn make_speak_dynamic(speaker: &dyn Speaker) {
    println!("(Dynamic) It says: {}", speaker.speak());
}
fn main() {
    let dog = Dog;
    let cat = Cat;

    // We can pass concrete types to both
    make_speak_static(&dog); // Compiler creates a version *just* for Dog
    make_speak_static(&cat); // Compiler creates a version *just* for Cat

    make_speak_dynamic(&dog); // Passes a reference to the dog
    make_speak_dynamic(&cat); // Passes a reference to the cat
}
