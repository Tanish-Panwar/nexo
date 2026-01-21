mod token;
mod lexer;

use lexer::Lexer;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: nx <file.nx>");
        return;
    }

    let source = fs::read_to_string(&args[1]).expect("Failed to read file");

    let mut lexer = Lexer::new(&source);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == token::Token::EOF {
            break;
        }
    }
}
