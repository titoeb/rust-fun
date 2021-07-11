fn main() {
    let v = vec![1, 2, 3];

    // If we would execute the following line, we could not use
    // the variable v anymore, because the ownership of the memory construct
    // on the heap will be passed to v2.
    //let v2 = v;
    println!("{:?}", v);

    // The same would happen with closures of function that use the vector.
    // A primitive solution is to return the reference afterwards.
    // Then we can store it it a new variable and have ownership over that
    // variable. Nevertheless, that process is tedious and that is why
    // `borrowing` exists.
    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        return x;
    };

    let vv = print_vector(v);
    println!("{:?}", vv);

    // Let's find a better way of doing this!
    let print_vector_borrow = |x: &Vec<i32>| {
        println!("{:?}", x);
    };

    print_vector_borrow(&vv);
    println!("{:?}", vv);

    let mut a = 40;
    {
        let b = &mut a;
        *b += 2;
    }
    println!("a: {}", a);

    let mut z = vec![3, 2, 1];
    for i in &mut z {
        println!("i: {}", i);
    }
}
