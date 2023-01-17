use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config = Config::build(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
