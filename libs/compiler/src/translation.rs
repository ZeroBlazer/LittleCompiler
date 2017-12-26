use std::sync::{Arc, Mutex};
use pest;
use Rule;
use std::fmt::Write;

#[derive(Debug, PartialEq)]
enum InstructionType {
    Move,
    Label,
    Sum,
    Mult,
    Return,
    Call,
    Print,
    Undef,
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

#[derive(Debug)]
struct InstructionTable {
    inst_vec: Vec<Instruction>,
    n_temp_elem: usize,
}

impl InstructionTable {
    fn new() -> InstructionTable {
        InstructionTable {
            inst_vec: Vec::new(),
            n_temp_elem: 0
        }
    }

    fn push(&mut self, instr: Instruction) {
        self.inst_vec.push(instr);
    }

    fn get_temp_name(&mut self) -> String {
        let ret = format!("_T{}", self.n_temp_elem);
        self.n_temp_elem += 1;
        ret
    }

    fn format_instructions(&self) -> String {
        let mut buf = String::new();

        for inst in &self.inst_vec {
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
}

use self::InstructionType::*;

/* *************************   RULES    **************************** */

fn translate_math_term(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<String, String> {

    let mut val = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::num | Rule::identifier => val = inner_pair.clone().into_span().as_str().to_string().trim_right().to_string(),
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ math_term", inner_pair.as_rule()),
        }
    }

    Ok(val)
}

fn translate_math_expr(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<String, String> {

    let mut ops = Vec::<String>::new();
    let mut instr = Undef;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::math_term => ops.push(translate_math_term(inner_pair, table.clone())?),
            Rule::num_op => {
                let oper = inner_pair.into_inner().nth(0).unwrap();
                
                instr = match oper.as_rule() {
                    Rule::op_add => Sum,
                    Rule::op_mult => Mult,
                    _ => {
                        println!("OPERATOR WAS NOT HANDLED!!: {:?} @ math_expr", oper.as_rule());
                        Undef
                    }
                }
            }
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ math_expr", inner_pair.as_rule()),
        }
    }

    if instr == Undef {
        Ok(ops.remove(0))
    } else {
        let temp = table.lock().unwrap().get_temp_name();
        table.lock().unwrap().push(Instruction::new(instr, temp.clone(), ops.remove(0), ops.remove(0)));
        Ok(temp)
    }
}

fn translate_func_call(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<String, String> {

    let mut instr = Call;
    let mut func_ident = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => func_ident = inner_pair.clone().into_span().as_str().to_string(),
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ func_call", inner_pair.as_rule()),
        }
    }

    let temp = table.lock().unwrap().get_temp_name();
    table.lock().unwrap().push(Instruction::new(instr, func_ident, temp.clone(), "".to_string()));
    Ok(temp)
}

fn translate_expr(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<String, String> {

    let mut val = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::math_expr => val = translate_math_expr(inner_pair, table.clone())?,
            Rule::func_call => val = translate_func_call(inner_pair, table.clone())?,
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ expr", inner_pair.as_rule()),
        }
    }

    // Ok(pair.into_span().as_str().to_string())
    Ok(val)
}

fn translate_right_assign(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<String, String> {

    let mut val = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::expr => val = translate_expr(inner_pair, table.clone())?,
            Rule::op_assign => {}
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ right_assign", inner_pair.as_rule()),
        }
    }

    Ok(val)
}

fn translate_var_decl(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<(), String> {

    let mut instr = Instruction::new(Move, "".to_string(), "".to_string(), "".to_string());

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => instr.op1.push_str(inner_pair.clone().into_span().as_str()),
            Rule::right_assign => instr.op2.push_str(translate_right_assign(inner_pair, table.clone())?.as_str()),
            Rule::semi => {}
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ var_decl", inner_pair.as_rule()),
        }
    }

    table.lock().unwrap().push(instr);

    Ok(())
}

fn translate_func_decl(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<(), String> {

    let mut val = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::identifier => {
                let mut instr = Instruction::new(Label, "".to_string(), "".to_string(), "".to_string());
                instr.op1.push_str(inner_pair.clone().into_span().as_str());
                table.lock().unwrap().push(instr);
            }
            Rule::statement => translate_statement(inner_pair, table.clone())?,
            Rule::expr => val = translate_expr(inner_pair, table.clone())?,
            Rule::block_start | Rule::block_end => {}
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ func_decl", inner_pair.as_rule()),
        }
    }

    table.lock().unwrap().push(Instruction::new(Return, val, "".to_string(), "".to_string()));

    Ok(())
}

fn translate_declaration(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<(), String> {

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::var_decl => translate_var_decl(inner_pair, table.clone())?,
            Rule::func_decl => translate_func_decl(inner_pair, table.clone())?,
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ declaration", inner_pair.as_rule()),
        }
    }

    Ok(())
}

fn translate_statement(
    pair: pest::iterators::Pair<Rule, pest::inputs::StrInput<'_>>,
    table: Arc<Mutex<InstructionTable>>,
) -> Result<(), String> {

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::declaration => translate_declaration(inner_pair, table.clone())?,
            Rule::expr => {
                let _temp = translate_expr(inner_pair, table.clone())?;
            }
            Rule::semi => {}
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ statement", inner_pair.as_rule()),
        }
    }

    Ok(())
}

pub fn translate_rustlin(
    pairs: pest::iterators::Pairs<Rule, pest::inputs::StrInput<'_>>,
) -> Result<String, String> {
    let instr_table = Arc::new(Mutex::new(InstructionTable::new()));

    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => translate_statement(pair, instr_table.clone())?,
            Rule::comment => {}
            _ => println!("CASE WAS NOT HANDLED!!: {:?} @ rustlin", pair.as_rule()),
        }
    }

    Ok(Arc::try_unwrap(instr_table).unwrap().into_inner().unwrap().format_instructions())
}
