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

union IntOrFloat {
    int: i32,
    float: f32,
}
fn process_value(int_of_float: IntOrFloat) {
    unsafe {
        match int_of_float {
            IntOrFloat { float } => {
                println!("value is {}", float);
            }
        }
    }
}

fn unions() {
    // Let's explore the world of unions in Rust.
    // Typically Unions are used to store different types of
    // values when it is not clear yet, which data type it will
    // be.
    let mut int_or_float = IntOrFloat { int: 123 };
    int_or_float.int = 234;

    // So far for the easy part, we assigned one of the two possible data types.
    // Let's see now how we can retrieve the value.
    let value = unsafe { int_or_float.int };
    println!("value={}", value);

    // Alright, with `unsafe` we pretty much declare that we are aware of what we are doing
    // and take responsibility if it goes wrong.

    // We could also write a dedicated funtion to handle the union:
    process_value(int_or_float);
}

fn option_t() {
    let x = 3.0;
    let y = 0.0;

    // We want to divide x by y. This can (and will with the allocated values) go wrong
    // if y happens to be 0. `Option<T>` gives us the chance to see whether a operation
    // suceeded or not.

    // Option -> Some(v) | None
    let result: Option<f32> = if y != 0.0 { Some(x / y) } else { None };

    // Great. But how can you now test the result?
    // 1) Use a match statement:
    match result {
        Some(value) => {
            println!("{} divided by {} is {}", x, y, value);
        }
        None => {
            println!("We cannot divide {} by {}", x, y);
        }
    }

    // 2) Use the `if let`:
    if let Some(z) = result {
        println!("result={}", z);
    }
}

fn arrays() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, the first one is {}.", a.len(), a[0]);

    // Let's try re-assignement.
    a[2] = 3;

    // This is how you could easily print out the entire array.
    println!("{:?}", a);

    // We can also use simply equal operators on the entire array.
    if a != [1, 2, 3, 4, 5] {
        println! {"{:?}", a};
    }

    // let's fill an aarray with a single value!
    let b = [1; 10];
    for idx in 0..b.len() {
        println!("elem {}: {}. ", idx, b[idx]);
    }

    // Let's talk mult-dimensional arrays!
    let matrix: [[f32; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    println!("{:?}", matrix);
    for row_idx in 0..matrix.len() {
        for col_idx in 0..matrix[row_idx].len() {
            if row_idx == col_idx {
                println!(
                    "The matrix at ({}, {}) is {}",
                    row_idx, col_idx, matrix[row_idx][col_idx]
                );
            }
        }
    }
}

fn main() {
    structs();
    enums();
    unions();
    option_t();
    arrays();
}
