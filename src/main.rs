use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        // eprintln macro to print error in stderr rather than stdout
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1)
    });

    // Using if-let in this case instead of unwrap_or_else
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1)
    }
}
