extern crate token_scanner;
extern crate ansi_term;

use std::fs::File;
use std::io::prelude::*;

mod validation;

use token_scanner::token_scanner;
use validation::validate_and_prefix;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compile(path: &str) {
    if let Ok(input) = load_file(path) {
        // Steps to compile file
        // 1. Tokenize input
        let tokens = token_scanner(&input);
        println!("TOKENS: {:?}\n", tokens);

        // 2. Validate grammar and prefix input
        match validate_and_prefix(tokens) {
            Ok(prefixed) => {
                // 3. Translate to assembly
                println!("{:?}", prefixed);
            }
            Err(errors) => {
                println!("Compilation had the following issues:");
                println!("{}", errors);
            }
        }
    } else {
        println!("Error while reading file! :/");
    }
}
