use std::io::{stdin, stdout, Write};

fn fahrenheit_to_celcius(temp_fahr: f64) -> f64 {
    (temp_fahr - 32.0) * (5.0 / 9.0)
}

fn main() {
    let mut input_string = String::new();
    print!("Please input the temperature in fahrenheit: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input_string)
        .expect("You did enter an incorrect string!");

    let temp_fahr: f64 = input_string.trim().parse().unwrap();
    println!(
        "{}F equals {}C",
        temp_fahr,
        fahrenheit_to_celcius(temp_fahr)
    );
}
