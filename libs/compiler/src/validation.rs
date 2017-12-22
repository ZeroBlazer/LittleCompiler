use std::collections::VecDeque;
use token_scanner::TokenType;
use ansi_term::Colour::*;

use TokenType::*;

fn eval_exp() {
	unimplemented!()
}

fn eval_statement() {
	unimplemented!()
}

fn eval_func_decl() {
	unimplemented!()
}

fn eval_var_decl() {
	unimplemented!()
}

fn eval_val_decl() {
	unimplemented!()
}

fn eval_rustlin_exp(tokens: VecDeque<(TokenType, String)>) -> bool {
	if tokens.len() == 0 {
		true
	} else {
		let 
		match tokens[0].0 {
			FnSt => {}
			
		}
	// if let Some(tokens) = eval_term(tokens) {
	// 	if let Some(tokens) = eval_resto(tokens) {
	// 		tokens.is_empty()
	// 	} else {
	// 		false
	// 	}
	// } else {
	// 	false
	// }
		false
	}
}

// fn eval_term(mut tokens: VecDeque<(TokenType, String)>) -> Option<VecDeque<(TokenType, String)>> {
// 	if tokens[0].0 != NUM {
// 		None
// 	} else {
// 		print!("{} ", tokens.pop_front().unwrap().1);
// 		Some(tokens)
// 	}
// }

pub fn validate_and_prefix(tokens: VecDeque<(TokenType, String)>) -> Result<(), String> {
    let mut errors = String::new();

	match eval_rustlin_exp(tokens) {
		Ok(_) => {}
		Err(eval_errors) => errors.push_str(eval_errors.as_str()),
	}

    errors.push_str(format!("{} text", Yellow.bold().paint("Some")).as_str());
    
    Err(errors)
}