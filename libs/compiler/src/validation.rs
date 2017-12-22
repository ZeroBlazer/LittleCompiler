use std::collections::VecDeque;
use token_scanner::TokenType;
use ansi_term::Colour::*;

pub fn validate_and_prefix(tokens: VecDeque<(TokenType, String)>) -> Result<(), String> {
    let mut errors = String::new();

    
    errors.push_str(format!("{} text", Yellow.bold().paint("Some")).as_str());
    
    Err(errors)
}