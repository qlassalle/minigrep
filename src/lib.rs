use std::error::Error;
use std::{env, fs};
use std::env::Args;

pub struct Config {
    pub pattern: String,
    pub filepath: String,
    pub ignore_case: bool,
}

const IGNORE_CASE_FLAG: &'static str = "IGNORE_CASE";

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(pattern) => pattern,
            None => return Err("Didn't get a pattern")
        };

        let filepath = match args.next() {
            Some(filepath) => filepath,
            None => return Err("Didn't get a filepath")
        };

        Ok(Config {
            pattern,
            filepath,
            ignore_case: env::var(IGNORE_CASE_FLAG).is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.pattern, &content)
    } else {
        search(&config.pattern, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
           .filter(|line| line.contains(query))
           .collect()
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines()
           .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
           .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
Fast, safe, productive
Embrace today
Duct tape";
        assert_eq!(vec!["Fast, safe, productive"], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rusT";
        let content = "\
Rust:
Fast, safe, productive
Embrace today
Trust me";

        assert_eq!(vec!["Rust:", "Trust me"], search_case_insensitive(query, content));
    }
}
