use std::{env, error::Error, fs};

// This structure "indicate" that file_path  and query are related and that they will be used for configuration purpose !
pub struct Config {
    pub file_path: String,
    pub query: String,
    pub ignore_case: bool,
}

// the main doesn't have the responsibility to determine how the command args and variable correspond !. The logic is outside the main scope.
// Using clone is not efficient but enable to simplify the code because the struct do not have to manage any lifetime.
// However it is making the code slower and less memory efficient (in term of space)
// Moreover we know that our file and thus its content is small so it is not a BIG issue here for this test project.

impl Config {
    //Constructor whcih handle error concerning hte nb of args.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments --> cargo run -- <file_path> <query>");
        }

        let file_path = args[1].clone();
        let query = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            file_path,
            query,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

//dyn for dynamic Error, can return different type of Error !
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value for the function to handle ! it is like return Err(_)
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

//Tell the compiler to execute this module for testing purpose!
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
