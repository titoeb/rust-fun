unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // Testing out rust unsafe's pointers.
    let mut num = 10;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("{}, {}", *r1, *r2);
        *r2 += 2;
        println!("{}, {}", *r1, *r2);
    }

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
