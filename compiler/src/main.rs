mod token;
mod lexer;
mod ast;
mod parser;
mod semantic;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use interpreter::Interpreter;
use token::Token;

use std::env;
use std::fs;

fn main() {
    let filename = &env::args().collect::<Vec<_>>()[1];
    let source = fs::read_to_string(filename).unwrap();

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

    let mut semantic = SemanticAnalyzer::new();
    semantic.analyze(&ast);

    let mut interpreter = Interpreter::new(&ast);
    interpreter.run();
}
