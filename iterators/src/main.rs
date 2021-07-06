fn main() {
    let mut my_vec: Vec<i32> = vec![3, 2, 1];

    for x in my_vec.iter() {
        println!("{}", *x);
    }

    for x in my_vec.iter().rev() {
        println!("in reverse: {}", *x);
    }

    for x in &my_vec {
        println!("{}", *x);
    }

    for x in &my_vec {
        println!("{}", *x);
    }

    for x in my_vec.iter_mut() {
        *x += 1;
        println!("{}", *x);
    }

    let mut my_vec_2 = vec![1, 2, 3];

    my_vec_2.extend(my_vec);

    println!("{:?}", my_vec_2);
}
