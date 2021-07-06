use std::collections::HashMap;

fn main() {
    // Let's create our first hash-map
    // The hash map is pretty much a python dict:
    let mut shapes: HashMap<String, i32> = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    // Now we can extract a value from the hash map with the
    // corresponding key:
    // But be aware, if the key does not exist this will a cause
    // a panic
    println!("A square has {} sides", shapes[&String::from("square")]);

    // We can easily iterate of the elements of the shape.
    for (key, value) in &shapes {
        println!("{} => {}", key, value);
    }

    // How can we add a new elements / update elements in the hash-map?
    shapes.insert(String::from("triangle"), 5);
    shapes.insert(String::from("circle"), 1);
    println!("{:?}", shapes);

    // We can also interactivly look for entries in the hash map
    // and if they don't exists we add them like so:
    shapes.entry(String::from("pentragram")).or_insert(5);

    println!("{:?}", shapes);
    let actual = shapes.entry(String::from("circle")).or_insert(2);
    *actual = 0;

    println!("{:?}", shapes);
}
