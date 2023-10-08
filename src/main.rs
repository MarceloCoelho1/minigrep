use std::env;

fn main() {
    println!("mingrep in rust");

    let args: Vec<String> = env::args().collect();

    println!("{:#?}", args);
}
