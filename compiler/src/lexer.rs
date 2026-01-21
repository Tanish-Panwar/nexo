use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.position].iter().collect()
    }

    fn read_string(&mut self) -> String {
        // Skip opening quote
        self.advance();

        let start = self.position;

        while let Some(c) = self.current_char() {
            if c == '"' {
                break;
            }
            self.advance();
        }

        let value: String = self.input[start..self.position].iter().collect();

        // Skip closing quote
        self.advance();

        value
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        let number: String = self.input[start..self.position].iter().collect();
        number.parse().unwrap()
    }


    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            Some('(') => { self.advance(); Token::LParen }
            Some(')') => { self.advance(); Token::RParen }
            Some('{') => { self.advance(); Token::LBrace }
            Some('}') => { self.advance(); Token::RBrace }
            Some(',') => { self.advance(); Token::Comma }
            Some(';') => { self.advance(); Token::Semicolon }
            Some('=') => {
                self.advance();
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            

            Some('+') => { self.advance(); Token::Plus }
            Some('-') => { self.advance(); Token::Minus }
            Some('*') => { self.advance(); Token::Star }
            Some('/') => { self.advance(); Token::Slash }

            Some(c) if c.is_ascii_digit() => {
                let num = self.read_number();
                Token::Int(num)
            }


            Some('"') => {
                let s = self.read_string();
                Token::String(s)
            }

            Some(c) if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "fn" => Token::Fn,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "return" => Token::Return,
                    "while" => Token::While,
                    _ => Token::Ident(ident),
                }
            }

            Some('>') => { self.advance(); Token::Greater }
            Some('<') => { self.advance(); Token::Less }


            None => Token::EOF,

            Some(_) => {
                self.advance();
                self.next_token()
            }
        }
    }
}
