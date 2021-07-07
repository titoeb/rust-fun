use rand::Rng;
use std::io::stdin;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    // Create a random number from 1-100
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        print!("Enter your guess: ");
        std::io::stdout().flush().unwrap();
        // Read in the guess.
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed: Result<i64, std::num::ParseIntError> = buffer.trim_end().parse();
                match parsed {
                    Ok(num) => {
                        if num < 1 || num > 100 {
                            println!("Your guess it out of range!");
                        } else if num > number {
                            println!("Your number is too high!");
                        } else if num < number {
                            println!("Your number is too small");
                        } else {
                            println!("Nice one, you number is spot-on!");
                            return;
                        }
                    }
                    Err(e) => {
                        println!("Your number could not be parsed, error was: {}", e);
                    }
                }
            }
            Err(_) => {}
        }
    }
}
