struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}
impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        return (dx * dx + dy * dy).sqrt();
    }
}

fn main() {
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let l1 = Line { start: p1, end: p2 };

    println!("length = {}", l1.len());
}
