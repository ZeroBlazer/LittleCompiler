#![feature(underscore_lifetimes)]
#![feature(string_retain)]

// extern crate token_scanner;
extern crate ansi_term;
extern crate pest;
#[macro_use] extern crate pest_derive;

use std::fs::File;
use std::io::prelude::*;
use pest::Parser;
use ansi_term::Colour::*;

#[derive(Parser)]
#[grammar = "rustlin.pest"]
struct RustlinParser;

mod translation;
mod interpreter;

use translation::translate_rustlin;
use interpreter::interpret;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compile(path: &str) {
    if let Ok(input) = load_file(path) {
        // Steps to compile file
        println!("{} \n", input);

        // Eval Grammar
        match RustlinParser::parse_str(Rule::input, &input) {
            Ok(pairs) => {
                let mut out = File::create("out/interm.ot").expect("Couldn't open write file");
                let instructions = translate_rustlin(pairs).unwrap();
                write!(out, "{}", instructions.format_instructions()).expect("Couldn't write output file");
                // println!("\nINSTRUCTION TABLE: {:#?}", instructions);
                interpret(instructions);
            }
            Err(e) => print!("{}\n{}",
                             Red.bold().paint("Parsing process failed at:"),
                             Yellow.bold().paint(format!("{}\n", e))),
        }
    } else {
        println!("Error while reading file! :/");
    }
}
