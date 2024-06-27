use std::{error::Error, fs};

// This structure "indicate" that file_path  and query are related and that they will be used for configuration purpose !
pub struct Config {
    pub file_path: String,
    pub query: String,
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

        Ok(Config { file_path, query })
    }
}

//dyn for dynamic Error, can return different type of Error !
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will return the error value for the function to handle ! it is like return Err(_)
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
