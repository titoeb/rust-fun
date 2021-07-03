use std::mem;
// Let's define global variables.
const MEANING_OF_LIFE: u8 = 42; // no fixed memory address, as it is only replaced
                                // for each occurence of `MEANING_OF_LIFE` in the code.

static Z: i32 = 123;

static mut T: i32 = 123;

fn main() {
    println!("{}", MEANING_OF_LIFE);
    println!("{}", Z);

    unsafe {
        println!("{}", T);
    }
}
