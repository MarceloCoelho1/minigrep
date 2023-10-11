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
    let contents = fs::read_to_string(config.file_path)?;


    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for Line {line, line_number} in &results {
        println!("{}: {}", line_number, line);
    }


    Ok(())
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<Line> {
    let query = query.to_lowercase();
    let mut results = Vec::<Line>::new();

    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for line in contents.lines() {
        line_number += 1;
        if line.to_lowercase().contains(&query) {
            _string_line = line_number;
            results.push(Line{line: String::from(line), line_number: _string_line});
        }
    }

    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<Line> {
    let mut results = Vec::new();
    
    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for line in contents.lines() {
        line_number += 1;
        if line.contains(query) {
            _string_line = line_number;
            results.push(Line{line: String::from(line), line_number: _string_line});
        }
    }

    results
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

}

