use std::env;
use std::process;

use mingrep;
use mingrep::Config;


fn main() {
    println!("mingrep in rust");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = mingrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }


}

