static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe{
        println!("COUNTER: {COUNTER}");
    }

    add_to_count(7);
    unsafe{
        println!("COUNTER: {COUNTER}");
    }

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    dbg!(r1);
    dbg!(r2);

    let address = 0x16b012345usize;
    let r = address as *const i32;
    dbg!(r);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        println!("r is: {}", *r);
    }
}

use std::slice;

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
