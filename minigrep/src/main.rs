use minigrep::Config;
use std::{env, process};

fn main() {
    //get the value in the args iterator and store them into a Vec<String> args.
    let args: Vec<String> = env::args().collect(); // <minigrep.exe cmd1 cmd2 ... cmdN>

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}",
        config.query, config.file_path
    );
    //run the app
    // no need to unwrap bcs we only care about the error no the value returned in case of success !
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    };
}
