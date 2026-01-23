mod token;
mod lexer;
mod ast;
mod parser;
mod semantic;
mod interpreter;
mod vm;
mod runtime_error;



use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use interpreter::Interpreter;
use token::Token;
use vm::{BytecodeCompiler, VM};


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
    let program = parser.parse_program();

    let mut semantic = SemanticAnalyzer::new();
    semantic.analyze(&program);

    // ---- VM PATH ----
    let compiler = BytecodeCompiler::new();
    let code = compiler.compile_program(&program);
    let mut vm = VM::new(code);
    vm.run();

    // ---- INTERPRETER (REFERENCE) ----
    // let mut interpreter = Interpreter::new(&program);
    // interpreter.run();
}
