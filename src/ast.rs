#[derive(Debug, Clone)]
pub enum NandScript {
    Literal(u8),
    Variable(String),
    ChipCall(String, Box<Vec<NandScript>>),
    ChipDef(String, Vec<String>, Box<NandScript>)
}