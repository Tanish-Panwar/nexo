#[derive(Debug, Clone)]
pub enum Instruction {
    // Stack ops
    PushInt(i64),
    PushString(String),
    LoadVar(String),
    StoreVar(String),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,

    // Comparison
    Less,
    Equal,
    Greater,

    // Control flow
    Jump(usize),
    JumpIfFalse(usize),

    // Function calls
    Call(String, usize), // name, argc
    Return,

    // Builtins
    Print,

    // Loop control
    Break,
    Continue,

    Halt,
}
