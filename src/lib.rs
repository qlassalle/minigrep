use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub pattern: String,
    pub filepath: String,
    pub ignore_case: bool
}

const IGNORE_CASE_FLAG: &'static str = "IGNORE_CASE";

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        Ok(Config {
            pattern: args[1].clone(),
            filepath: args[2].clone(),
            ignore_case: env::var(IGNORE_CASE_FLAG).is_ok()
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
    let mut matches = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }

    matches
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
