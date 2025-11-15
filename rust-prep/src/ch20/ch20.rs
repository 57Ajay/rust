// the unsafe shenanigans

pub fn main() {
    println!("Hello from ch20");

    unsafe_main(); // we can say this is a safe abstraction over
                   // unsafe code as it have a unsafe function call;
    let mut data = [2, 3, 6, 7, 9, 2, 1];
    println!("split: {:?}", split_at_mut(&mut data, 3));
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
