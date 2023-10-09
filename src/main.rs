use std::env;
use std::process;

use mingrep::Config;


fn main() {
    println!("mingrep in rust");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    Config::run(config).expect("You cannot read this file");

}

