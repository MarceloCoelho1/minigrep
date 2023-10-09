use std::env;
use std::fs;
use std::process;

struct Config{
    _query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: &str = &args[1];
        let file_path: &str = &args[2];
    
        Ok(Config{
            _query: query.to_string(),
            file_path: file_path.to_string(),
        })
    }
    
}

fn main() {
    println!("mingrep in rust");

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_content = fs::read_to_string(config.file_path).expect("You cannot read this file");

    println!("File Content: \n{file_content}");
}

