struct Person {
    name: String,
}

struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

impl Person {
    fn get_ref_name(&self) -> &String {
        return &self.name;
    }
}

fn main() {
    let boss = Person {
        name: "Idiot".to_string(),
    };
    let idiots = Company {
        name: "Tesla".to_string(),
        ceo: &boss,
    };

    let mut z: &String;
    {
        let p = Person {
            name: String::from("John"),
        };
        z = p.get_ref_name();
    }
}
