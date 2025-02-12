use std::env;
use std::process;
use minigrep_12::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}",  err);
        process::exit(1);
    });
    config.log();

    if let Err(e) = minigrep_12::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1); 
    }
}
