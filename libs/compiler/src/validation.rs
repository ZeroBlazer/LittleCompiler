use std::collections::VecDeque;
use token_scanner::TokenType;
use ansi_term::Colour::*;

pub fn validate_and_prefix(tokens: VecDeque<(TokenType, String)>) -> Result<(), String> {
    let errors = String::new();
    
    Err(format!("{} text", Yellow.bold().paint("Some")))
}