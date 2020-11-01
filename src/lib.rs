use std::error::Error;
use std::fs;
use std::env;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive_str = env::var("CASE_INSENSITIVE").expect("Expect Error");
        let mut case_sensitive= false;
        if !case_sensitive_str.eq("1"){
            case_sensitive = true;
        }
        Ok(Config {
            query,
            filename,
            case_sensitive
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename).expect("Something went wrong when read from file");
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else {
        search_case_insensitive(&config.query,&contents)
    };
    for line in results {
        println!("{}",line)
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
    #[test]
    fn p () {
        let mut array_a = [1,2,3];
        c(array_a);
        fn c (mut array_b: [i32;3]) {
            array_b[1] = 2;
        }
        println!("{:?}",array_a)
    }
    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."],search(query, content))
    }
}
pub fn search<'a>(query: &str, contents: & 'a str) -> Vec<& 'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}