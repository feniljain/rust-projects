use minigrep;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }
}
