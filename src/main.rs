use std::env;
use std::fs;

fn main() {
    println!("mingrep in rust");

    let args: Vec<String> = env::args().collect();

    let query: &str = &args[1];
    let file_path: &str = &args[2];

    let file_content = fs::read_to_string(file_path).expect("You cannot read this file");


    println!("{:#?}", args);

    println!("Searching for: {}", query);
    println!("In file: {}", file_path);


    println!("File Content: \n{file_content}");
}
