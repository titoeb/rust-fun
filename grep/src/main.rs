use std::env;
use std::process;

use grep::Config;

fn main() {
    // Collect input arguments
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!(
            "The following problem occured when parsing the arguments:\n{}",
            err
        );
        process::exit(1);
    });

    if let Err(err) = grep::run(config) {
        eprintln!("Execution failed with: {:?}", err);
        process::exit(1);
    };
}
