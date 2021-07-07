fn print_value(x: i32) {
    println!("{}", x);
}

fn increase_side_effects(x: &mut i32) {
    *x += 1;
}

fn increase(x: i32) -> i32 {
    return x + 1;
}

fn functions() {
    let mut x = 33;
    print_value(x);
    increase_side_effects(&mut x);
    print_value(x);
    x = increase(x);
    print_value(x);
}

fn main() {
    functions();
}
