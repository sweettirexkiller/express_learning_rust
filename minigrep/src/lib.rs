use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Functionality to be implemented
    let contents = fs::read_to_string(config.filename)?;

    for line in search_case_sensitive(config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query: query,
            filename: filename,
        })
    }
}

pub fn search_case_sensitive<'a>(query: String, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: String, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct".to_string();

        let contests = "\
        Rust: safe, fast, productive.
        Pick three.
        duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contests));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT".to_string();

        let contests = "\
        Rust: safe, fast, productive.
        Trust me.";

        assert_eq!(
            vec!["Rust: safe, fast, productive.", "Trust me."],
            search_case_insensitive(query.to_lowercase(), &contests.to_lowercase())
        );
    }
}
