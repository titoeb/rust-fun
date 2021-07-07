fn main() {
    // This is the way we create a  string
    // it is a slice of a static string that cannot
    // be changed.
    let my_string: &'static str = "Hello there!";
    println!("{}", my_string);

    // We can also iterate over the string:
    for my_char in my_string.chars() {
        println!("{}", my_char);
    }
    // It is important to see above that `my_char`  is not a reference to the char
    // that is part of the static string.
    // We could also access a single char from the slice to a static string like so:
    if let Some(first_char) = my_string.chars().nth(0) {
        println!("First letter is: {}", first_char);
    }

    // But there is also an alternative type for string: String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(", ");
        a += 1;
    }
    println!("letters: {:?}", letters);

    // Great! But how can we convert between &str <> String?

    // Fairly simple!
    let u: &str = &letters;

    // Whats about concatenation?
    // String + &str
    let z = letters + my_string;
    println!("{:?}", z);

    let d = String::from("testing") + &String::from(" still ...");
    println!("{:?}", d);
    let iter_string = String::from("This is what I am iterating about ...");

    for my_char in iter_string.chars() {
        println!("{}", my_char);
    }

    // Let's now focus on the format macro!
    // The format macro is very similar to the `println!`, the main
    // difference is that the `println!` macro prints to stdout, while
    // the `format!` macro returns a string!
    let name = "Tim";
    let greeting = format!("Hi, I'm {}, nice to meet you!", name);
    println!("{:?}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    // We can also add more information inside the curly brackets
    // to specify which variable to print!
    let hello_rust_hello = format!("{0}, {1}, {0}", hello, rust);
    println!("{}", hello_rust_hello);

    //
    let info = format!(
        "the name's {last}. {first} {last}.",
        first = "janina",
        last = "bond"
    );
    println!("{}", info);
}
