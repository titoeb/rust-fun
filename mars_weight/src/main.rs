use std::io;
use std::io::Write; // <--- bring the trait into scope
fn main() {

    // Try to read earth-weight from stdin.
    let mut input = String::new();
    print!("Please type in your weight on earth: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut input){
        Ok(_) => {},
        Err(_) => {
            println!("Your value could not be read. Restart execution");
            main();
        }
    }

    // Try to read float from stdin string.
    let weight_on_earth: f32 = match input.trim().parse(){
        Ok(val) => {val},
        Err(_) => {
            println!("The value you entered is not a string, default to 100.0kg.");
            100.0
        }
    };

    // Convert earth-weight to mars weight and print out result.
    println!("weight on mars = {} kg.", calculate_weight_on_mars(weight_on_earth));
}

fn calculate_weight_on_mars(weight_on_earth: f32) -> f32{
    (weight_on_earth / 9.81) * 3.711
}