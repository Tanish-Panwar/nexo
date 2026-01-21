mod token;
mod lexer;
mod ast;
mod parser;
mod semantic;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use codegen::CodeGenerator;
use token::Token;

use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let source = fs::read_to_string(filename).unwrap();

    // Lexing
    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    loop {
        let t = lexer.next_token();
        tokens.push(t.clone());
        if t == Token::EOF {
            break;
        }
    }

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();

    // Semantic
    let mut semantic = SemanticAnalyzer::new();
    semantic.analyze(&ast);

    // Codegen
    let c_code = CodeGenerator::new().generate(&ast);

    fs::write("out.c", &c_code).expect("Failed to write C output");

    // Compile C → EXE
    let status = Command::new("gcc")
        .args(["out.c", "-o", "out.exe"])
        .status()
        .expect("Failed to invoke gcc");

    if !status.success() {
        panic!("C compilation failed");
    }

    println!("✔ Build successful: out.exe");
}
