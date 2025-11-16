// the unsafe shenanigans

pub fn main() {
    println!("Hello from ch20");

    unsafe_main(); // we can say this is a safe abstraction over
                   // unsafe code as it have a unsafe function call;
    let mut data = [2, 3, 6, 7, 9, 2, 1];
    println!("split: {:?}", split_at_mut(&mut data, 3));
    overload();
}

fn unsafe_main() {
    raw_ptr();
    let x = unsafe { unsafe_func_meth() }; // boom, a unsafe function;
    println!("Val: {:?}", x);
}

// raw pointers
fn raw_ptr() {
    let mut x = 69;
    let _y = &raw const x;
    let _z = &raw mut x;
    let address = 0x0124usize;
    let _r = address as *const i32; // *r = 69; is not possible as it is behing
                                    // *const
    unsafe { *_z = 69 };
    // println!("{:?}", unsafe { *_r }); // will give segementation fault
    println!("{:?}", unsafe { *_z });
}

// calling unsafe function or methods

unsafe fn unsafe_func_meth() -> i32 {
    let mut x: i32 = 69;
    let y = &raw mut x;
    unsafe {
        *y = 6969;
        return *y;
    }
}

// Safe Abstraction over Unsafe Code

// use std::slice;

#[allow(dead_code)]
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // let len = values.len();
    // let ptr = &raw mut values[0];
    //
    // assert!(mid <= len);
    //
    // unsafe {
    //     (
    //         slice::from_raw_parts_mut(ptr, mid),
    //         slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    //     )
    // }
    // wrinting my own impl

    let my_slice = as_my_slice(values);
    unsafe {
        let (left, right) = my_slice.split_at_mut(mid);

        (from_my_slice_to_slice(left), from_my_slice_to_slice(right))
    }
}

use std::marker::PhantomData;
use std::mem::size_of;

#[derive(Debug)]
struct MySliceMut<'a, T> {
    ptr: *mut T,
    len: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> MySliceMut<'a, T> {
    unsafe fn new(ptr: *mut T, len: usize) -> Self {
        Self {
            ptr,
            len,
            _marker: PhantomData,
        }
    }

    unsafe fn offset_ptr(ptr: *mut T, i: usize) -> *mut T {
        ((ptr as usize) + i * size_of::<T>()) as *mut T
    }

    unsafe fn split_at_mut(self, mid: usize) -> (Self, Self) {
        assert!(mid <= self.len);

        let left = MySliceMut::new(self.ptr, mid);

        let right_ptr = Self::offset_ptr(self.ptr, mid);
        let right = MySliceMut::new(right_ptr, self.len - mid);

        (left, right)
    }
}

fn as_my_slice<'a>(values: &'a mut [i32]) -> MySliceMut<'a, i32> {
    // let ptr = values.as_mut_ptr();
    let ptr = &raw mut values[0];
    let len = values.len();

    unsafe { MySliceMut::new(ptr, len) }
}

unsafe fn from_my_slice_to_slice<'a, T>(slice: MySliceMut<'a, T>) -> &'a mut [T] {
    // Slice fat pointer layout: (data_ptr, len)
    #[repr(C)]
    struct FatPtr<T> {
        data: *mut T,
        len: usize,
    }

    let fp = FatPtr {
        data: slice.ptr,
        len: slice.len,
    };

    // reinterpret struct pointer as *mut [T]

    let slice_ptr = std::mem::transmute::<FatPtr<T>, *mut [T]>(fp);

    &mut *slice_ptr
}

// Advanced traits;

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // it feels like operator overloading
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Meters(u32);
#[derive(Debug, PartialEq)]
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn dog() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

fn overload() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let x = Meters(69);
    let y = Millimeters(1000);
    let z = y + x;
    assert_eq!(z, Millimeters(70000));
    dog();
    println!("all overload asserted");
    let pp = PP { x: 12, y: 21 };
    pp.outline_print(); // PP::outline_print(&pp) was also possible
    newpattern();
}

// super traits
use std::fmt::{self, Display, Result};
// the Result is a type alias ->
// type Result<T> = std::result::Result<T, std::io::Error>;
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PP {
    x: u8,
    y: u8,
}

impl OutlinePrint for PP {}

impl Display for PP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// newtype pattern ->  let’s say we want to implement Display on Vec<T>,
// which the orphan rule prevents us from doing directly because the
// Display trait and the Vec<T> type are defined outside our crate.
// We can make a Wrapper struct that holds an instance of Vec<T>;
// then we can implement Display on Wrapper and use the Vec<T> value,
// as shown below (this comment is from the rust book ->
// https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits
// )

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn newpattern() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
    adv_type();
}

// Advanced types
// ! ->  the never type

fn adv_type() {
    fn_as_param();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn fn_as_param() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
    println!("{:?}", list_of_strings);

    opaque();
}

// Advanced types -> Opaque types
fn opaque() {
    println!("from opaque");
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
    test_my_vec();
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// Advanced types -> macros

// Declarative Macros with macro_rules! for General Metaprogramming
#[macro_export]
macro_rules! my_vec {
    ( $ ( $x:expr ),* ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x);
            )*
            temp
        }
    };
}

fn test_my_vec() {
    let x = my_vec![1, 3, 5, 2];
    println!("{:?}", x);
}

// Procedural Macros for Generating Code from Attributes

pub trait HelloMacro {
    fn hello_macro();
}

// use proc_macro::TokenStream;
// use quote::quote;
//
// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
//     // Construct a representation of Rust code as a syntax tree
//     // that we can manipulate.
//     let ast = syn::parse(input).unwrap();
//
//     // Build the trait implementation.
//     impl_hello_macro(&ast)
// }
// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
//     let name = &ast.ident;
//     let generated = quote! {
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}!", stringify!(#name));
//             }
//         }
//     };
//     generated.into()
// }
