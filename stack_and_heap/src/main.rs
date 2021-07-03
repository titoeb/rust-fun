use std::mem;
static var: i32 = 32;
struct Point {
    x: f64,
    y: f64,
}
fn origin() -> Point {
    Point { x: 1.0, y: 2.0 }
}

fn main() {
    // Let's allocate a point on the stack.
    let p1: Point = origin();

    // Now let's save it on the heap.
    let p2 = Box::new(origin());

    // Putting it on the heap of course creates additional overhead,
    // because in addition to the object itself (the two floating-point values)
    // of the point we need to store the actual pointer, e.g. the reference
    // to the object on the heap.
    println!("size_of_val(p1) == {}", mem::size_of_val(&p1));
    println!("size_of_val(p2) == {}", mem::size_of_val(&p2));

    // Let's put the point that p2 is pointing to back on the stack.
    let p3 = *p2;
    println!("{}", p3.x);
}
