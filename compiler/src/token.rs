#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Fn,
    Let,
    If,
    Else,
    For,
    Return,

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

    // Operators
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /

    // Special
    EOF,
}
