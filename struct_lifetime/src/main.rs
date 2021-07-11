struct Person<'a> {
    name: &'a str,
}

impl<'a> Person<'a> {
    fn talk(&self) -> String {
        return format!("Hi, my name is {}.", self.name);
    }
}

fn main() {
    let person = Person { name: "Tim" };
    println!("{person}", person = person.talk());
}
