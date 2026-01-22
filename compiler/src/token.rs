#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    If,
    Else,
    For,
    Return,
    While,
    Break, 
    Continue,

    // Identifiers & literals
    Ident(String),
    Int(i64),
    String(String),

    // Symbols
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    Comma,    // ,
    Semicolon,// ;
    Equal, // =
    
    // Comparison Ops T-T
    Greater,   // >
    Less,      // <
    EqualEqual,// ==



    // Operators
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /

    // Special
    EOF,
}
