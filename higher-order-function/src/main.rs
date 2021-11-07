type BasicMathOp = fn(t: usize) -> usize;

fn add(x: usize) -> usize{
    println!("Executing `add`.");
    x + x
}
fn multiply(x: usize) -> usize{
    println!("Executing `multiply`.");
    x * x
}
fn add_or_multiple(add: bool, on_add: BasicMathOp, on_multply: BasicMathOp, t: usize) -> usize{
    if add{
        on_add(t)
    } else {
        on_multply(t)
    }
}

fn main() {
    println!("{}", add_or_multiple(true, add, multiply, 4));
    println!("{}", add_or_multiple(false, add, multiply, 4));
}
