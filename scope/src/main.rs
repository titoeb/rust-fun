fn scope_and_shadowing() {
    let a = 123;

    // Let's create a new scope!
    {
        let b = 456;
        println!("inside, b={}", b);
        // Because of shadowing, you still have the variable a:
        println!("outisde: a={}", a);

        // we will overshadow the outer a.
        let a = 20;
        println!("inside: a={}", a);
    }

    // Now b does not exist any more as the scope has closed.
    println!("a={}", a);
}

fn main() {
    scope_and_shadowing();
}
