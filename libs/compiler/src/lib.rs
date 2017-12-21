use std::fs::File;
use std::io::prelude::*;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compile(path: &str) {
    if let Ok(input) = load_file(path) {
        println!("{}", input);
    } else {
        println!("Error while reading file!");
    }
}
