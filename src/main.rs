use std::env;
use std::fs;


struct Config{
    _query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config{
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query: &str = &args[1];
        let file_path: &str = &args[2];
    
        Config{
            _query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }
    
}

fn main() {
    println!("mingrep in rust");

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args);

    let file_content = fs::read_to_string(config.file_path).expect("You cannot read this file");

    println!("File Content: \n{file_content}");
}

