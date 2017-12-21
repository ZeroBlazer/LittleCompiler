extern crate compiler;

use std::env;

fn main() {
    if env::args().len() != 2 {
        println!("Usage: compiler <filename.c>");
    } else {
        compiler::compile(&env::args().nth(1).unwrap());
    }
}
