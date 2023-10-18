use std::error::Error;
use std::fs;
use std::env;



pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Line {
    line: String,
    line_number: u32,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query: &str = &args[1];
        let file_path: &str = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config{
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })
    }

    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {


    if config.ignore_case {
        search_case_insensitive(&config)
    } else {
        search_case_sensitive(&config)
    }


    Ok(())
}

pub fn search_in_all_files<'a> (config: &Config){
    let entries = fs::read_dir(&config.file_path).expect("Error");



    for entry in entries {
        let entry = entry.expect("Error");
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Imprime o nome de cada arquivo no diretório
        println!("{}", file_name_str);
    }

    

}

pub fn search_case_insensitive<'a> (config: &Config) {
    let file = fs::read_to_string(&config.file_path).expect("Error");
    let query = config.query.to_lowercase();
    let mut results = Vec::<Line>::new();

    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for line in file.lines() {
        line_number += 1;
        if line.to_lowercase().contains(&query) {
            _string_line = line_number;
            results.push(Line{line: String::from(line), line_number: _string_line});
        }
    }

    for Line {line, line_number} in &results {
        println!("{}: {}", line_number, line);
    }
}

pub fn search_case_sensitive<'a>(config: &Config) {
    let file = fs::read_to_string(&config.file_path).expect("Error");
    let mut results = Vec::new();
    let query = &config.query;

    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for line in file.lines() {
        line_number += 1;
        if line.contains(&*query) {
            _string_line = line_number;
            results.push(Line{line: String::from(line), line_number: _string_line});
        }
    }

    for Line {line, line_number} in &results {
        println!("{}: {}", line_number, line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected_results = vec![
            Line {
                line: "Rust:".to_string(),
                line_number: 1,
            },
            Line {
                line: "Trust me.".to_string(),
                line_number: 4,
            }
        ];

        assert_eq!(expected_results, search_case_insensitive(query, contents));
    }


    #[test]
    fn case_sensitive() {
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected_results = vec![
            Line {
                line: "Rust:".to_string(),
                line_number: 1,
            }
        ];

        assert_eq!(expected_results, search_case_sensitive(query, contents));
    }

}

