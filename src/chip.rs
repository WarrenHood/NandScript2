use std::{collections::HashMap, iter::zip};

use crate::ast::NandScript;

pub struct Chip {
    args: Vec<String>,
    logic: NandScript
}

impl Chip {
    pub fn new(args: Vec<String>, logic: NandScript) -> Self {
        Self { args: args, logic: logic }
    }
}

pub struct CPU {
    chips: HashMap<String, Chip>
}

impl CPU {
    pub fn new() -> Self {
        Self { chips: HashMap::new() }
    }

    pub fn load_chip(&mut self, chipdef: NandScript) {
        if let NandScript::ChipDef(chip_name, args, logic) = chipdef {
            self.chips.insert(chip_name, Chip::new(args, *logic));
        } 
        else {
            panic!("Cannot add non chipdef to chip db!");
        };
    }

    pub fn run_nandscript(&self, ns: &NandScript, arg_names: Vec<String>, args: Vec<NandScript>) -> u8{
        match ns {
            NandScript::Literal(x) => *x,
            NandScript::Variable(arg_name) => {
                for (a_name, a) in zip(arg_names.clone(), args.clone()) {
                    if a_name == *arg_name {
                        return self.run_nandscript(&a, arg_names, args);
                    }
                }
                panic!("Couldn't find variable {}", &arg_name);
            },
            NandScript::ChipCall(chip_name, chipargs) => {
                let chipargs = chipargs.iter().map(|arg| self.run_nandscript(arg, arg_names.clone(), args.clone()));
                return self.run_chip(chip_name, chipargs.collect());
            },
            NandScript::ChipDef(_, _, _) => todo!(),
        }
    }

    pub fn run_chip(&self, chip_name: &str, args: Vec<u8>) -> u8 {
        if chip_name == "NAND" {
            let mut result: u8 = 1;
            for arg in args.iter() {
                result = result & arg;
            }
            return !result;
        }

        let chip = self.chips.get(chip_name).expect(&format!("Couldn't find chip definition for chip {}", &chip_name));
        self.run_nandscript(&chip.logic, chip.args.clone(), args.iter().map(|x| NandScript::Literal(*x)).collect())
    }
}