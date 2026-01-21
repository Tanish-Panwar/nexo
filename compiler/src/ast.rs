#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    ExprStmt(Expr),
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    IntLiteral(i64),
    VarRef(String),
    StringLiteral(String),
}



#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Greater,
    Less,
    Equal,
}
