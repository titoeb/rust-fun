fn add_n(n: usize) -> impl Fn(usize) -> usize{
    move |y| -> usize{n + y}
}

fn main() {
    println!("{}", add_n(10)(5));
    println!("{}", add_n(20)(5));
    println!("{}", add_n(30)(5));
}


