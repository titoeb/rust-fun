use std::convert::TryFrom;
use std::io::{stdin, stdout, Write};
fn gen_nth_fib_number(n: i64) -> i64 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else if n == 2 {
        2
    } else {
        gen_nth_fib_number(n - 1) + gen_nth_fib_number(n - 2)
    }
}

fn gen_nth_fib_number_fast(n: i64) -> Option<i64> {
    let mut my_fibs: Vec<i64> = vec![1, 2];
    if n <= 0 {
        None
    } else {
        while n > i64::try_from(my_fibs.len()).unwrap() {
            my_fibs.push(my_fibs[my_fibs.len() - 1] + my_fibs[my_fibs.len() - 2]);
        }
        Some(my_fibs[usize::try_from(n - 1).unwrap()])
    }
}
// fn main() {
//     let mut input_string = String::new();
//     println!("Generate the nth fibonacci number.");
//     print!("Please input n: ");
//     let _ = stdout().flush();
//     stdin()
//         .read_line(&mut input_string)
//         .expect("You number was not valid!");
//     let n: i64 = input_string.trim().parse().unwrap();
//     println!("The {}th fibonacci number is {}", n, gen_nth_fib_number(n));
// }
fn main() {
    let mut input_string = String::new();
    println!("Generate the nth fibonacci number.");
    print!("Please input n: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input_string)
        .expect("You number was not valid!");
    let n: i64 = input_string.trim().parse().unwrap();
    if let Some(n_th_fib) = gen_nth_fib_number_fast(n) {
        println!("The {}th fibonacci number is {}", n, n_th_fib);
    } else {
        println!("The number you inputed is not a valid interger. Please try again!");
    }
}
