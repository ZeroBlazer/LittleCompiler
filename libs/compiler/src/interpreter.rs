use std::collections::BTreeMap;
use translation::InstructionTable;
use translation::InstructionType::*;
use std::collections::btree_map::Entry::{Vacant, Occupied};

#[derive(Debug, Clone)]
enum Val {
    // Int(i32),
    Float(f32),
    Func(usize, usize),
}

use self::Val::*;

#[derive(Debug)]
struct Stack {
    map: BTreeMap<String, Val>
}

impl Stack {
    fn new() -> Stack {
        Stack {
            map: BTreeMap::new(),
        }
    }

    fn insert_var(&mut self, ident: String, val: Val) {
        self.map.insert(ident, val).expect("Couldn't insert var to stack");
    }

    fn get_val(&self, ident: String) -> Option<Val> {
        if let Some(val) = self.map.get(&ident) {
            Some(val.clone())
        } else {
            if let Ok(val) = ident.as_str().parse::<f32>() {
                Some(Float(val))
            } else {
                None
            }
        }
    }

    fn update_val_str(&mut self, ident: String, val: String) {
        if let Ok(val) = val.as_str().parse::<f32>() {
            match self.map.entry(ident) {
                Vacant(entry) => {
                    entry.insert(Float(val));
                }
                Occupied(mut entry) => {
                    entry.insert(Float(val));
                }
            }
        } else {
            let oth_val = self.map.get(&val).unwrap().clone();
            match self.map.entry(ident) {
                Vacant(entry) => {
                    entry.insert(oth_val);
                }
                Occupied(mut entry) => {
                    entry.insert(oth_val);
                }
            }
        }
    }

    fn update_val(&mut self, ident: String, val: Val) {
        match self.map.entry(ident) {
            Vacant(entry) => {
                entry.insert(val);
            }
            Occupied(mut entry) => {
                entry.insert(val);
            }
        }
    }
}

pub fn interpret(instr_tab: InstructionTable) {
    let mut pc = 0;
    let limit = instr_tab.instructions_size();
    let mut stack = Stack::new();

    let mut in_label = false;
    let mut interval = (0, 0);

    while pc < limit {
        let inst = instr_tab.get_instruction_at(pc).unwrap();

        match inst.instr {
            Move => if !in_label {
                    stack.update_val_str(inst.op1.clone(), inst.op2.clone());
                },
            Label => {
                in_label = true;
                interval.0 = pc + 1;
            }
            Sum => if !in_label {
                if let (Some(term1), Some(term2)) = (stack.get_val(inst.op2.clone()), stack.get_val(inst.op3.clone())) {
                    if let (Float(term1), Float(term2)) = (term1, term2) {
                        stack.update_val(inst.op1.clone(), Float(term1 + term2));
                    } else {
                        panic!("Operation {:?} supported only for numeric types", inst.instr);
                    }
                } else {
                    panic!("Not valid Operation {:?}", inst.instr);
                }
            },
            Mult => if !in_label {
                if let (Some(term1), Some(term2)) = (stack.get_val(inst.op2.clone()), stack.get_val(inst.op3.clone())) {
                    if let (Float(term1), Float(term2)) = (term1, term2) {
                        stack.update_val(inst.op1.clone(), Float(term1 * term2));
                    } else {
                        panic!("Operation {:?} supported only for numeric types", inst.instr);
                    }
                } else {
                    panic!("Not valid Operation {:?}", inst.instr);
                }
            },
            Return => {
                in_label = false;
                interval.1 = pc;

                stack.update_val(inst.op1.clone(), Func(interval.0, interval.1));
            }
            // Call => {

            // }
            // Print => {}
            _ => println!("Instruction {:?} was IGNORED", inst.instr),
        }
        pc += 1;
    }

    println!("STACK: {:#?}", stack);
}