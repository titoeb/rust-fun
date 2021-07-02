use std::mem;

fn main() {
    // Create a integer.
    let a: u8 = 123; // unsigned integer with 8 bits, range: 0-255
    println!("a = {}", a);

    // Create a mutuable version of this number:
    let mut b: i8 = 0; // signed integer with 8 bits: range -128 to 127
    println!("b = {}", b);
    b = 10;
    println!("b = {}", b);

    let mut z = 123456789; // This variable type will be infered from the value we assign first.
                           // In this particular case this will be i32
    println!("z = {}, and takes up {} bytes.", z, mem::size_of_val(&z));
    z = -1;
    println!("z = {}", z);

    // usize, isize
    let c: isize = 123;
    let size_of_c = mem::size_of_val(&c);
    println!(
        "c = {}, takes up {} bytes, {}-bit OS",
        c,
        size_of_c,
        size_of_c * 8,
    );

    // let's get into characters.
    let d: char = 'x';
    println!(
        "d = {}, size_of_val(&d) = {} bytes",
        d,
        mem::size_of_val(&d)
    );

    // And whats about floats?
    // -> There are f32 and f64 IEEE754, ofc signed.
    let e: f32 = 2.5;
    println!("e={}, mem::size_of_val(&e)={}", e, mem::size_of_val(&e));
    let f = 2.5;
    println!("f={}, mem::size_of_val(&f)={}", f, mem::size_of_val(&f));

    // and finally, there are boolean:
    let g: bool = true;
    println!("g={}, sizeof g ={}", g, mem::size_of_val(&g));
}
