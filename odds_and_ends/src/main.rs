extern crate rand;
use rand::Rng;

extern crate phrases;
use phrases::greetings::english;
fn main() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
    println!("{}", b);
    println!("English: {}", english::hello());
}
