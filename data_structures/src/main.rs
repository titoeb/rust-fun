struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structs() {
    let first_point: Point = Point { x: 3.0, y: 4.5 };
    println!("Point p is at ({}, {})", first_point.x, first_point.y);

    let second_point: Point = Point { x: 12.0, y: 13.0 };
    let my_line: Line = Line {
        start: first_point,
        end: second_point,
    };
}

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8), // tuple
    Cmyk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // struct
}

fn enums() {
    let my_color: Color = Color::Red;
    match my_color {
        Color::Red => {
            println!("The color is red!");
        }
        Color::Green => {
            println!("The color is green!");
        }
        Color::Blue => {
            println!("The color is blue!");
        }
        Color::RGBColor(0, 0, 0) => {
            println!("The line is black!");
        }
        Color::RGBColor(r, g, b) => {
            println!(
                "The color is RBG and has the following components: ({}, {}, {})",
                r, g, b
            );
        }
        Color::Cmyk {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => {
            println!("black");
        }
        Color::Cmyk {
            cyan: c,
            magenta: m,
            yellow: y,
            black: b,
        } => {
            println!(
                "The color is a Cmyk color with components: ({}, {}, {}, {})",
                c, m, y, b
            );
        }
    }
}

fn main() {
    structs();
    enums();
}
