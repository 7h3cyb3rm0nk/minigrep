use std::error::Error;
use std::fs;
use std::env;
use std::collections::HashMap;


pub struct Config {
   pub query: String,
   pub file_path: String,
   pub ignore_case: bool,
}

impl Config {
    pub fn build ( 
            mut args: impl Iterator< Item = String>,
          )  -> Result<Config, &'static str> {


        args.next();

        let query = match args.next() {
            some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE")is_ok();



        Ok(Config {
            query,
            file_path,
            ignore_case,
        })

    }
}
        
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }
    else {
        search(&config.query, &contents)
    };

    for (number, line) in results.iter() {
        println!("found \"{line}\" on line: {number}");
    }
    println!("\n");

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three,";

        let lines: Vec<&str> = search(query, contents).into_values().collect();
        let num: Vec<i32> = search(query, contents).into_keys().collect();
        assert_eq!(vec!["safe, fast, productive."], lines );
        assert_eq!(vec![2], num);
    }
    #[test]
    fn case_insensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three,";
        
        let lines: Vec<&str> = search_case_insensitive(query, contents).into_values().collect();
        let num: Vec<i32> = search_case_insensitive(query, contents).into_keys().collect();
        assert_eq!(vec!["safe, fast, productive."], lines );
        assert_eq!(vec![2], num);                            
    }
}

pub fn search_case_insensitive<'a>
(query: &str, contents: &'a str) -> HashMap<i32, &'a str>  {
    let mut count = 0;
    let mut results = HashMap::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        count += 1;
        if line.to_lowercase().contains(&query) {
            results.insert(count, line);
        }
    }
    results
    
}

        

pub fn search<'a>(query: &str, contents: &'a str) -> HashMap<i32, &'a str> {
    let mut results = HashMap::new();
    let mut count = 0;
    for line in contents.lines() {
        count += 1;
        if line.contains(query) {
            results.insert(count, line);
        }
    }

    results

    
}
