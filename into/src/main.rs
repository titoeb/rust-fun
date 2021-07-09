use std::fmt::Debug;

#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<S: Into<String>>(name: S) -> Person {
        return Person { name: name.into() };
    }
}

fn main() {
    let john = Person::new("John");
    let name: String = "Jane".to_string();
    let jane = Person::new(name);
    println!("{:?}", jane);
    println!("{:?}", john);
}
