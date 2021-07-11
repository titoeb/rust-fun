use std::rc::Rc;

struct Person {
    name: Rc<String>,
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        return Person { name: name };
    }
    fn greet(&self) -> Rc<String> {
        {
            return self.name.clone();
        };
    }
}

fn main() {
    let name = Rc::new("Tim".to_string());
    println!("name: {}, n_references: {}", name, Rc::strong_count(&name));
    {
        let person = Person::new(name.clone());
        println!("name: {}, n_references: {}", name, Rc::strong_count(&name));
        println!("Hi my name is {}.", person.greet());
    }
    println!("name = {}", name);
    println!("name: {}, n_references: {}", name, Rc::strong_count(&name));
}
