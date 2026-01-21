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

    // =======================
    // PROGRAM
    // =======================

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

    // =======================
    // STATEMENTS
    // =======================

    fn parse_statement(&mut self) -> Stmt {
        match self.current() {
            Token::Let => self.parse_let(),
            _ => {
                let expr = self.parse_expression();
                self.expect(Token::Semicolon);
                Stmt::ExprStmt(expr)
            }
        }
    }

    fn parse_let(&mut self) -> Stmt {
        self.expect(Token::Let);

        let name = if let Token::Ident(n) = self.current() {
            let n = n.clone();
            self.advance();
            n
        } else {
            panic!("Expected variable name");
        };

        self.expect(Token::Equal);

        let value = self.parse_expression();

        self.expect(Token::Semicolon);

        Stmt::Let { name, value }
    }

    // =======================
    // EXPRESSIONS (PRECEDENCE)
    // =======================

    fn parse_expression(&mut self) -> Expr {
        self.parse_add_sub()
    }

    // + -
    fn parse_add_sub(&mut self) -> Expr {
        let mut expr = self.parse_mul_div();

        loop {
            match self.current() {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_mul_div();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Add,
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_mul_div();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Sub,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    // * /
    fn parse_mul_div(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        loop {
            match self.current() {
                Token::Star => {
                    self.advance();
                    let right = self.parse_primary();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Mul,
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_primary();
                    expr = Expr::Binary {
                        left: Box::new(expr),
                        op: BinOp::Div,
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        expr
    }

    // literals, identifiers, calls, parentheses
    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Token::Int(value) => {
                let v = *value;
                self.advance();
                Expr::IntLiteral(v)
            }

            Token::Ident(name) => {
                let name = name.clone();
                self.advance();

                // function call
                if *self.current() == Token::LParen {
                    self.advance();

                    let mut args = Vec::new();
                    while *self.current() != Token::RParen {
                        args.push(self.parse_expression());
                        if *self.current() == Token::Comma {
                            self.advance();
                        }
                    }

                    self.expect(Token::RParen);
                    Expr::Call { name, args }
                } else {
                    Expr::VarRef(name)
                }
            }

            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(Token::RParen);
                expr
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
