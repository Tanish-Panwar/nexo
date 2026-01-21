#[derive(Debug)]
pub struct Program {
    pub functions: Vec<FunctionDecl>,
}

#[derive(Debug)]
pub struct FunctionDecl {
    pub name: String,
    pub body: Block,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Call {
        name: String,
        args: Vec<Expr>,
    },
    StringLiteral(String),
}
