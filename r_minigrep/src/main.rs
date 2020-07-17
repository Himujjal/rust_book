use std::{env, process};

use r_minigrep::{run, Config};

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for ... {}", config.query);
    // println!("in file ... {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
