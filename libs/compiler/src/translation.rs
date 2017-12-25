use std::sync::{Arc, Mutex};
use pest;
use Rule;

struct Instruction {
    instr: i32,
    op1: i32,
    op2: i32,
    op3: i32
}

impl Instruction {
    fn new(ins: i32, op1: i32, op2: i32, op3: i32) -> Instruction {
        Instruction {
            instr: ins,
            op1: op1,
            op2: op2,
            op3: op3
        }
    }
}

fn translate_statement(pairs: pest::iterators::Pairs<Rule, pest::inputs::StrInput<'_>>,
                       table: ) -> Result<(), String> {

}

pub fn translate_rustlin(pairs: pest::iterators::Pairs<Rule, pest::inputs::StrInput<'_>>) -> Result<(), String> {
    let instr_table = Arc::new(Mutex::new(Vec::<Instruction>new()))

    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                println!(">>>>>>>>>>>>> {:?}", pair);
            }
            _ => {
                println!("{:?}", pair);
            }
        }
    }

    Ok(())
}