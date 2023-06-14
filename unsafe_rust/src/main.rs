// rust sunsafe superpowers
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access or modify a mutable static variable
// 4. Implement an unsafe trait
// 5. Access fields of unions

fn example_1() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

use std::slice;
fn unsafe_splicing() {

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
        }
    }
}

fn main() {
    println!("Hello, world!");
}
