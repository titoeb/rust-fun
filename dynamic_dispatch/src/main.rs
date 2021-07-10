struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        return self.side * self.side;
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

fn main() {
    let shapes: [&dyn Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 3.0 },
        &Circle { radius: 2.0 },
        &Square { side: 4.0 },
    ];

    for (idx, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", idx, shape.area())
    }

    enum MyShape {
        Circle(Circle),
        Square(Square),
    }

    // Let's put both Sides and Squares into shapes!
    let mut more_shapes = Vec::new();
    more_shapes.push(MyShape::Square(Square { side: 3.0 }));
    more_shapes.push(MyShape::Circle(Circle { radius: 3.0 }));

    // Let's try it without an additional enum.
    let mut other_shapes: Vec<Box<dyn Shape>> = Vec::new();
    other_shapes.push(Box::new(Square { side: 3.0 }));
    other_shapes.push(Box::new(Circle { radius: 3.0 }));

    for this_shape in other_shapes.iter() {
        println!("{}", this_shape.area());
    }
}
