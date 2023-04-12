mod ast;
mod parser;
mod chip;
use std::io::{BufRead, Write};
use chip::CPU;
use parser::*;


fn main() {
    let mut cpu: CPU = CPU::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut buf: String = String::new();
        std::io::stdin().lock().read_line(&mut buf).unwrap();
        let (_, ns) = parse_nandscript(&buf).unwrap();
        match ns {
            ast::NandScript::ChipDef(_, _, _) => {
                cpu.load_chip(ns);
            },
            _ => {
                let result = cpu.run_nandscript(&ns, vec![], vec![]);
                println!("0b{0:08b} | 0x{0:02x} | {0}", &result);
            }
        }
    }
    
}
