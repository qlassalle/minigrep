use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Looking for {} in {}", config.pattern, config.filepath);
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error from app: {e}");
        process::exit(1);
    }
}
