use std::error::Error;
use std::fs;
use std::env;
use std::path::Path;
use std::borrow::Cow;

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
    file_name: Option<String>,
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

 

    if config.file_path.contains("/*.") {
        search_in_all_files(&config)
    } else if config.ignore_case {
        search_case_sensitive(&config)
    } else {
        search_case_insensitive(&config)
    }


    Ok(())
}

pub fn search_in_all_files<'a> (config: &Config){

    let path = Path::new(&config.file_path);
    let file_path = path.parent().unwrap();


    let extension = path.extension().unwrap().to_str().unwrap();
    let _extension: Cow<'_, str> = Cow::Borrowed(extension);




    let entries = fs::read_dir(&file_path).expect("Error");
    let mut results = Vec::<Line>::new();
    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for entry in entries {

        let entry = entry.expect("Error");
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        
        if file_name_str.ends_with(extension) {
            let contents = fs::read_to_string(&path).expect("Error");
            
            for line in contents.lines() {
                line_number += 1;
                
                if line.to_lowercase().contains(&config.query.to_lowercase()) {
                    _string_line = line_number;
                    results.push(Line{line: String::from(line), line_number: _string_line, file_name: Some(file_name_str.to_string())});
                }
            }
            line_number = 0;
        };
        


    }

   

    for Line {line, line_number, file_name} in &results {
        println!("{:?} Line: {} {}", file_name, line_number, line);
    }

    

}

pub fn search_case_insensitive<'a> (config: &Config) {
    let contents = fs::read_to_string(&config.file_path).expect("Error");
    let query = config.query.to_lowercase();
    let mut results = Vec::<Line>::new();

    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for line in contents.lines() {
        line_number += 1;
        if line.to_lowercase().contains(&query) {
            _string_line = line_number;
            results.push(Line{line: String::from(line), line_number: _string_line, file_name: None});
        }
    }

    for Line {line, line_number, file_name: _} in &results {
        println!("{}: {}", line_number, line);
    }
}

pub fn search_case_sensitive<'a>(config: &Config) {
    let contents = fs::read_to_string(&config.file_path).expect("Error");
    let mut results = Vec::new();
    let query = &config.query;

    let mut line_number: u32 = 0;
    let mut _string_line: u32 = 0;

    for line in contents.lines() {
        line_number += 1;
        if line.contains(&*query) {
            _string_line = line_number;
            results.push(Line{line: String::from(line), line_number: _string_line, file_name: None});
        }
    }

    for Line {line, line_number, file_name: _} in &results {
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

