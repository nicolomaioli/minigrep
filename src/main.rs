use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args = env::args(); // env::args() returns an iterator

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {:?}", e);
        process::exit(1);
    }
}
