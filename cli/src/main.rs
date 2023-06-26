use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let config: Config = Config::new(&arguments).unwrap_or_else(|err | {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    }
    );

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);
    
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box <dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_name: String
}

impl Config {
    fn new(arguments: &[String]) -> Result<Config, &str> {
        if arguments.len() < 3 {
            return Err("not enough arguments")
        }
        let query: String = arguments[1].clone();
        let file_name: String = arguments[2].clone();
        Ok(Config {query, file_name})
    }
}