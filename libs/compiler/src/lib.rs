#![feature(underscore_lifetimes)]
#![feature(string_retain)]

// extern crate token_scanner;
extern crate ansi_term;
extern crate pest;
#[macro_use] extern crate pest_derive;

use std::fs::File;
use std::io::prelude::*;
use pest::Parser;

#[derive(Parser)]
#[grammar = "rustlin.pest"]
struct RustlinParser;

mod validation;
use validation::validate_rustlin;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compile(path: &str) {
    if let Ok(input) = load_file(path) {
        // Steps to compile file
        println!("{}  \n", input);

        // Eval Grammar
        match RustlinParser::parse_str(Rule::input, &input) {
            Ok(pairs) => {
                validate_rustlin(pairs);
                // println!("{:?} ", pairs);
                // for pair in pairs {
                //     match pair.as_rule() {
                //         Rule::statement => {
                //             println!(">>>>>>>>>>>>> {:?}", pair);
                //         }
                //         _ => {
                //             println!("{:?}", pair);
                //         }
                //     }
                // }
                // pairs.for_each(|p| println!("R_{:?}: [{:?}] -> {}", p.as_rule(), p.clone().into_span(), p.clone().into_span().as_str()));
            }
            Err(e) => panic!("{}", e),
        }
    } else {
        println!("Error while reading file! :/");
    }
}
