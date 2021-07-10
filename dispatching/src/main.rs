trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        return format!("i32: {}", *self);
    }
}

impl Printable for String {
    fn format(&self) -> String {
        return format!("String: {}", *self);
    }
}

// Static Dispatch, monomorphisation:
// At compile time for all applicable traits one of these
// method (with the correct types) will be created.
// All usage of the these functions in the code will then
// be replaced with the correct implementation for the specific types.
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

// Dynamic Dispatch
// Here, at time, it has to be determined, what underlying
// class called this function (what is the type of z) and
// then the respective method is called.
// Therefore, this is more expensive.
fn print_it_too(z: &dyn Printable) {
    println!("{}", z.format());
}

fn main() {
    let a: i32 = 123;
    let b: String = "hello".to_string();
    print_it(a);
    print_it(b);
    let a: i32 = 123;
    let b: String = "hello".to_string();
    print_it_too(&a);
    print_it_too(&b);
}
