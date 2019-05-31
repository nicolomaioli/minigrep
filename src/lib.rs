use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for result in search(&config.query, &contents) {
        println!("{}", result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_returns_one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn search_returns_multiple_results() {
        let query = "unc";
        let contents = "Punctuation\nuncharted territories.";

        assert_eq!(
            vec!["Punctuation", "uncharted territories."],
            search(query, contents)
        );
    }

    #[test]
    fn search_returns_no_result() {
        let query = "morph";
        let contents = "Punctuation\nuncharted territories.";
        let result: Vec<&str> = Vec::new();

        assert_eq!(
            result,
            search(query, contents)
        );
    }
}
