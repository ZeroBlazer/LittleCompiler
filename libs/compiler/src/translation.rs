use std::sync::{Arc, Mutex};
use pest;
use Rule;
use std::fmt::Write;

#[derive(Debug)]
enum InstructionType {
    Move,
    Print,
}

#[derive(Debug)]
struct Instruction {
    instr: InstructionType,
    op1: String,
    op2: String,
    op3: String,
}

impl Instruction {
    fn new(ins: InstructionType, op1: String, op2: String, op3: String) -> Instruction {
        Instruction {
            instr: ins,
            op1: op1,
            op2: op2,
            op3: op3,
        }
    }
}

use self::InstructionType::*;

fn translate_expr(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<Vec<Instruction>>>,
) -> Result<String, String> {
    println!("\nRULE: {:?}", pair.as_rule());
    println!("PAIR: {:?}", pair);

    // let mut val = String::new();

    // for inner_pair in pair.into_inner() {
    //     println!(">>> Inner: {:?}", inner_pair.as_rule());

    //     match inner_pair.as_rule() {
    //         Rule::expr => {
    //             val = inner_pair.clone().into_span().as_str().to_string();
    //         }
    //         _ => {
    //             println!("CASE WAS NOT HANDLED!!: {:?}", inner_pair.as_rule());
    //         }
    //     }
    // }

    Ok(pair.into_span().as_str().to_string())
}

fn translate_right_assign(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<Vec<Instruction>>>,
) -> Result<String, String> {
    // println!("\nRULE: {:?}", pair.as_rule());
    // println!("PAIR: {:?}", pair);

    let mut val = String::new();

    for inner_pair in pair.into_inner() {
        // println!(">>> Inner: {:?}", inner_pair.as_rule());

        match inner_pair.as_rule() {
            Rule::expr => {
                val = translate_expr(inner_pair, table.clone())?;
                println!("VAL {}", val);
            }
            _ => {
                println!("CASE WAS NOT HANDLED!!: {:?}", inner_pair.as_rule());
            }
        }
    }

    Ok(val)
}

fn translate_var_decl(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<Vec<Instruction>>>,
) -> Result<(), String> {
    // println!("\nRULE: {:?}", pair.as_rule());
    // println!("PAIR: {:?}", pair);

    let mut instr = Instruction::new(Move, "".to_string(), "".to_string(), "".to_string());

    for inner_pair in pair.into_inner() {
        // println!(">>> Inner: {:?}", inner_pair.as_rule());

        match inner_pair.as_rule() {
            Rule::identifier => {
                instr.op1.push_str(inner_pair.clone().into_span().as_str());
                println!("{}", inner_pair);
            }
            Rule::right_assign => {
                instr.op2.push_str(translate_right_assign(inner_pair, table.clone())?.as_str());
            }
            _ => {
                println!("CASE WAS NOT HANDLED!!: {:?}", inner_pair.as_rule());
            }
        }
    }

    table.lock().unwrap().push(instr);

    Ok(())
}

fn translate_declaration(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<Vec<Instruction>>>,
) -> Result<(), String> {
    // println!("\nRULE: {:?}", pair.as_rule());
    // println!("PAIR: {:?}", pair);

    for inner_pair in pair.into_inner() {
        // println!(">>> Inner: {:?}", inner_pair.as_rule());

        match inner_pair.as_rule() {
            Rule::var_decl => {
                translate_var_decl(inner_pair, table.clone())?;
            }
            _ => {
                println!("CASE WAS NOT HANDLED!!: {:?}", inner_pair.as_rule());
            }
        }
    }

    Ok(())
}

fn translate_statement(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<Vec<Instruction>>>,
) -> Result<(), String> {
    // println!("\nRULE: {:?}", pair.as_rule());
    // println!("PAIR: {:?}", pair);

    for inner_pair in pair.into_inner() {
        // println!("Inner: {:?}", inner_pair);

        match inner_pair.as_rule() {
            Rule::declaration => {
                translate_declaration(inner_pair, table.clone())?;
            }
            _ => {
                println!("CASE WAS NOT HANDLED!!: {:?}", inner_pair.as_rule());
            }
        }
    }

    Ok(())
}

fn format_instructions(table: Vec<Instruction>) -> String {
    let mut buf = String::new();

    for inst in &table {
        writeln!(
            buf,
            "{:?}\t{}\t{}\t{}",
            inst.instr,
            inst.op1,
            inst.op2,
            inst.op3
        ).expect("Error while writing instructions");
    }

    buf
}

pub fn translate_rustlin(
    pairs: pest::iterators::Pairs<Rule, pest::inputs::StrInput<'_>>,
) -> Result<String, String> {
    let instr_table = Arc::new(Mutex::new(Vec::<Instruction>::new()));

    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                translate_statement(pair, instr_table.clone())?;
            }
            _ => {
                println!("CASE WAS NOT HANDLED!!: {:?}", pair.as_rule());
            }
        }
    }

    Ok(format_instructions(
        Arc::try_unwrap(instr_table).unwrap().into_inner().unwrap(),
    ))
}
