#[derive(Debug, Clone)]
pub enum Instruction {
    PushInt(i64),
    PushString(String),
    LoadVar(String),
    StoreVar(String),
    PushVoid,

    Add,
    Sub,
    Mul,
    Div,

    Less,
    Equal,
    Greater,

    Call(String, usize),
    Return,

    Print,
    Pop,

    Jump(usize),
    JumpIfFalse(usize),

    Halt,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub entry: usize, // instruction index
}

pub struct Program {
    pub functions: Vec<Function>,
    pub code: Vec<Instruction>,
}