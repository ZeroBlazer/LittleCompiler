extern crate token_scanner;

use std::fs::File;
use std::io::prelude::*;

use token_scanner::token_scanner;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compile(path: &str) {
    if let Ok(input) = load_file(path) {
        // Steps to compile file
        let tokens = token_scanner(&input);
        println!("{:?}", tokens);
    } else {
        println!("Error while reading file!");
    }
}
