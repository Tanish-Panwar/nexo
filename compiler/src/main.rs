mod token;
mod lexer;
mod ast;
mod parser;

use lexer::Lexer;
use parser::Parser;
use token::Token;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let source = fs::read_to_string(&args[1]).unwrap();

    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();

    loop {
        let t = lexer.next_token();
        tokens.push(t.clone());
        if t == Token::EOF {
            break;
        }
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    println!("{:#?}", ast);
}
