use bitvec::prelude::*;

#[derive(Debug, Clone)]
pub enum NandScript {
    Literal(BitVec),
    Variable(String),
    ChipCall(String, Box<Vec<NandScript>>),
    ChipDef(String, Vec<String>, Box<NandScript>)
}