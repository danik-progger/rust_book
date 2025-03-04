use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    let results: Vec<&str> = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the query"),
        };
        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the file path"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            file_name: file_name,
            case_sensitive: case_sensitive,
        })
    }
    pub fn log(&self) {
        println!("Query: {}\nFile Name: {}", self.query, self.file_name);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust
safe, fast, productive
Pick three
Trust me";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust
safe, fast, productive
Pick three
Trust me";
        assert_eq!(
            vec!["Rust", "Trust me"],
            search_case_sensitive(query, contents)
        )
    }
}
