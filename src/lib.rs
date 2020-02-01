use std::fs;
use std::io::{self, Read};
mod validation;

pub struct Config {
    pub filename: Option<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1 => Ok(Config { filename: None }),
            2 => Ok(Config {
                filename: Some(args[1].clone()),
            }),
            _ => Err("invalid amount of arguments"),
        }
    }
}

/// Tries to read the input. Will either read from stdin if config.filename is None, otherwise it
/// will try to open the given file.
///
/// @param {Config} config
///
/// @return {Result<String, String>} - either the contents of the file or stdin, otherwise an error
/// string

fn read_contents(config: Config) -> Result<String, String> {
    match config.filename {
        None => {
            // Read from stdin
            let mut buf = String::new();
            match io::stdin().lock().read_to_string(&mut buf) {
                Ok(_) => Ok(buf),
                Err(e) => Err(format!("Problem reading from stdin: {}", e)),
            }
        }
        Some(s) => {
            // Read from file
            match fs::read_to_string(s) {
                Ok(s) => Ok(s),
                Err(e) => Err(format!("Problem opening file: {}", e)),
            }
        }
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let contents: String;

    match read_contents(config) {
        Ok(c) => contents = c,
        Err(e) => return Err(e),
    }
    println!("{}", contents);
    match validation::validate_contents(&contents) {
        Err(e) => return Err(e),
        Ok(m) => println!("{:?}", m),
    }
    Ok(())
}
