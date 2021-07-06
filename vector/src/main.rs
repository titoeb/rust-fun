fn vectors() {
    // Opposed to the array, a vector can grow in size.
    // and there is almost mutuable "by default":
    let mut my_vec = Vec::new();
    // At least a non-mutuable vector can't grow in size ;)

    // Then you can use `push` to add elements to the vector.
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    println!("my_vec= {:?}", my_vec);

    // But you can also access specific elements of the vector:
    println!("my_vec[0]= {}", my_vec[0]);

    // What is the type of index variables (as `0` above)?
    // -> We use usize for that!
    // usize is an unsigned integer that is default in your current system
    // e.g. 64 bit -> u64, 32 bit -> u32
    let idx: usize = 2;
    println!("my_vec[{}]= {}", idx, my_vec[idx]);

    // If the index is out of bounds, this will cause a panic.
    // An alternative to get elements savely is to use the `get`
    // function that returns an option.
    match my_vec.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("a[6] does not exist."),
    }

    // This is how you would iterate over a vector:
    for val in &my_vec {
        println!("{}", val);
    }

    // Finally, how can we actually delete elements from a vector?
    // We can `.pop` the elements from the end!
    let last_elem = match my_vec.pop() {
        Some(x) => x,
        None => -1,
    };
    println!("The extracted last element is {}", last_elem);
    println!("The vector now looks like {:?}", my_vec);

    // Let's put the value back in there:
    my_vec.push(3);
    while let Some(x) = my_vec.pop() {
        println!("{}", x);
    }

    // And if we print the vector, we'll see it is empty now:
    println!("{:?}", my_vec);
}

fn main() {
    vectors()
}
