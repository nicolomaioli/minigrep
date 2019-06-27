use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // The first value in args is the name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not present"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename not present"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines() // returns an iterator
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines() // returns an iterator
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    // Assign search function according to config
    let search_fn = if config.case_sensitive {
        search
    } else {
        search_case_insensitive
    };

    let results = search_fn(&config.query, &contents);

    // If no results, display some useful information
    if results.len() < 1 {
        println!("Query '{}' not found in {}", config.query, config.filename);
        println!("Search case sensitive: {}", config.case_sensitive);
        return Ok(());
    }

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_returns_one_result() {
        let query = "Pun";
        let contents = "Punctuation\nuncharted territories.";

        assert_eq!(vec!["Punctuation"], search(query, contents));
    }

    #[test]
    fn test_search_returns_multiple_results() {
        let query = "unc";
        let contents = "Punctuation\nuncharted territories.";

        assert_eq!(
            vec!["Punctuation", "uncharted territories."],
            search(query, contents)
        );
    }

    #[test]
    fn test_search_returns_no_result() {
        let query = "morph";
        let contents = "Punctuation\nuncharted territories.";
        let result: Vec<&str> = Vec::new();

        assert_eq!(result, search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "pun";
        let contents = "Punctuation\nuncharted territories.";

        assert_eq!(
            vec!["Punctuation"],
            search_case_insensitive(query, contents)
        );
    }
}
