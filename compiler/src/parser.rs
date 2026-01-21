use crate::ast::*;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, token: Token) {
        if *self.current() == token {
            self.advance();
        } else {
            panic!("Expected {:?}, got {:?}", token, self.current());
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut functions = Vec::new();

        while *self.current() != Token::EOF {
            functions.push(self.parse_function());
        }

        Program { functions }
    }

    fn parse_function(&mut self) -> FunctionDecl {
        self.expect(Token::Fn);

        let name = if let Token::Ident(n) = self.current() {
            let n = n.clone();
            self.advance();
            n
        } else {
            panic!("Expected function name");
        };

        self.expect(Token::LParen);
        self.expect(Token::RParen);
        self.expect(Token::LBrace);

        let body = self.parse_block();

        self.expect(Token::RBrace);

        FunctionDecl { name, body }
    }

    fn parse_block(&mut self) -> Block {
        let mut statements = Vec::new();

        while *self.current() != Token::RBrace {
            statements.push(self.parse_statement());
        }

        Block { statements }
    }

    fn parse_statement(&mut self) -> Stmt {
        let expr = self.parse_expression();
        self.expect(Token::Semicolon);
        Stmt::ExprStmt(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        match self.current() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();

                self.expect(Token::LParen);

                let mut args = Vec::new();

                while *self.current() != Token::RParen {
                    args.push(self.parse_expression());
                    if *self.current() == Token::Comma {
                        self.advance();
                    }
                }

                self.expect(Token::RParen);

                Expr::Call { name, args }
            }

            Token::String(value) => {
                let v = value.clone();
                self.advance();
                Expr::StringLiteral(v)
            }

            _ => panic!("Unexpected token {:?}", self.current()),
        }
    }
}
