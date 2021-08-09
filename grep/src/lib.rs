use std::error::Error;
use std::fs;

fn usage(file_name: &String) -> String {
    format!("USAGE: {} <query-string> <file-name>", file_name)
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(format!("Too few arguments provided\n{}", usage(&args[0])));
        }

        // Safely extract string and filename,
        // show usage pattern if on of them does not exist.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read in file.
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // Search query in file
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut found_matches: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            found_matches.push(line);
        }
    }
    found_matches
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
