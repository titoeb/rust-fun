struct Point<T> {
    x: T,
    y: T,
}

fn generics() {
    let float_point: Point<f64> = Point { x: 0.5, y: 0.5 };
    let int_point: Point<i32> = Point { x: 1, y: 2 };
}

fn main() {
    generics();
}
