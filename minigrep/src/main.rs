use std::env;
use std::process;

use minigrep::Config;

fn main() {
    /*
     * Use collect to convert an iterator to collection
     * Type annotation is a must while using collect since
     * Rust won't be able to figure out what type of collection
     * we want
     */
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }    
}
