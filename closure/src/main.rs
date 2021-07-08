fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 {
        return x + 1;
    };
    let a = 5;
    println!("{} + 1 = {}", a, plus_one(a));
    let plus_two = |x: i32| {
        return x + 2;
    };
    println!("{} + 2 = {}", a, plus_two(a));

    let plus_three = |x: &mut i32| {
        *x += 2;
    };

    let mut f = 12;
    plus_three(&mut f);
    println!("{}", f);
    let plus_three = |mut x: i32| {
        x += 2;
    };

    let mut g = 12;
    plus_three(g);
    println!("{}", g);
}
fn main() {
    closures();
}
