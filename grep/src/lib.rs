use std::env;
use std::error::Error;
use std::fs;

fn usage(file_name: &String) -> String {
    format!("USAGE: {} <query-string> <file-name>", file_name)
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read in file.
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut found_matches: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
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
    #[test]
    fn case_sensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        let empty_vec: Vec<String> = Vec::new();
        assert_eq!(empty_vec, search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust_me.";
        assert_eq!(
            vec!["Rust:", "Trust_me."],
            search_case_insensitive(query, contents)
        )
    }
}
