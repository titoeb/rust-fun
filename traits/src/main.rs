trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}
struct Human {
    name: &'static str,
}
impl Animal for Human {
    fn create(name: &'static str) -> Human {
        return Human { name: name };
    }

    fn name(&self) -> &'static str {
        return &self.name;
    }

    fn talk(&self) {
        println!("{} says hello.", &self.name);
    }
}
struct Cat {
    name: &'static str,
}
impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        return Cat { name: name };
    }

    fn name(&self) -> &'static str {
        return &self.name;
    }

    fn talk(&self) {
        println!("{} says meow.", &self.name);
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for val in self {
            sum += *val;
        }
        return sum;
    }
}

fn main() {
    // Now we can use the `create` method that is defined by each trait
    // to create an object of type Human and cat respectivly, and let it speak.
    let human = Human::create("tim");
    human.talk();

    let cat = Cat::create("Missy");
    cat.talk();

    // Now this is cool:
    // Rust chooses the correct implementation (that is Human::create()) because
    // of the type we assigned test as!
    let test: Human = Animal::create("test");
    test.talk();

    let a: Vec<i32> = vec![1, 2, 3];
    println!("{}", a.sum());
}
