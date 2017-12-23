#![feature(string_retain)]

// extern crate token_scanner;
extern crate ansi_term;
extern crate pest;
#[macro_use] extern crate pest_derive;

use std::fs::File;
use std::io::prelude::*;
use pest::Parser;

// mod validation;

// use token_scanner::token_scanner;
// use validation::validate_and_prefix;

#[derive(Parser)]
#[grammar = "rustlin.pest"]
struct RustlinParser;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compile(path: &str) {
    if let Ok(input) = load_file(path) {
        // Steps to compile file
        // input.retain(|c| c != '\n');
        println!("{} \n", input);
        match RustlinParser::parse_str(Rule::input, &input) {
            Ok(pairs) => {
                println!("{:?} ", pairs);
                pairs.for_each(|p| println!("R_{:?}: [{:?}] -> {}", p.as_rule(), p.clone().into_span(), p.clone().into_span().as_str()));
                // println!("{:#?}", pairs);
            }
            Err(e) => panic!("{}", e),
        }


        // let tokens = token_scanner(&input);
        // println!("TOKENS: {:?}\n", tokens);

        // // 2. Validate grammar and prefix input
        // match validate_and_prefix(tokens) {
        //     Ok(prefixed) => {
        //         // 3. Translate to assembly
        //         println!("{:?}", prefixed);
        //     }
        //     Err(errors) => {
        //         println!("Compilation had the following issues:");
        //         println!("{}", errors);
        //     }
        // }
    } else {
        println!("Error while reading file! :/");
    }
}
