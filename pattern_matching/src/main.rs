fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "A dozen oranges",
        _z @ 9..=11 => "oranges", // You could use z now to operate on the range.
        _ if (x % 2 == 0) => "some oranges",
        _ => "a few",
    }
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }
}

fn main() {
    pattern_matching();

    let point: (i32, i32) = (3, 4);
    match point {
        (0, 0) => println!("This is the origin!"),
        (0, x) => println!("x axis, x = {}", x),
        (y, 0) => println!("y axis, y = {}", y),
        _ => println!("Arbitrary point!"),
    }

    color();
}
