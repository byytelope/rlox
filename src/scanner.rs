use crate::{token::Token, token_type::TokenType};

pub struct Scanner {
    src: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: String) -> Self {
        Self {
            src,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(TokenType::Eof, "", self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let ttype = if self.match_advance('=') {
                    TokenType::Bang
                } else {
                    TokenType::BangEqual
                };

                self.add_token(ttype)
            }
            '=' => {
                let ttype = if self.match_advance('=') {
                    TokenType::Equal
                } else {
                    TokenType::EqualEqual
                };

                self.add_token(ttype)
            }
            '<' => {
                let ttype = if self.match_advance('=') {
                    TokenType::Less
                } else {
                    TokenType::LessEqual
                };

                self.add_token(ttype)
            }
            '>' => {
                let ttype = match self.match_advance('=') {
                    true => TokenType::Greater,
                    false => TokenType::GreaterEqual,
                };

                self.add_token(ttype)
            }
            '/' => {
                if self.match_advance('/') {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            '"' => self.handle_string(),
            _ => {
                if c.is_ascii_digit() {
                    self.handle_number();
                } else if c.is_ascii_alphabetic() {
                    self.handle_identifier();
                } else {
                    eprintln!("Unexpected character on line {}: {}", self.line, c);
                }
            }
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&self) -> char {
        self.src
            .chars()
            .nth(self.current + 1)
            .expect("Error while peeking in advance()...")
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.src.as_str()[self.start..self.current];
        self.tokens.push(Token::new(token_type, text, self.line))
    }

    fn match_advance(&mut self, expected: char) -> bool {
        if self.at_end() {
            return false;
        }

        if self
            .src
            .chars()
            .nth(self.current)
            .expect("Error while peeking in match_advance()...")
            != expected
        {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }

        self.src
            .chars()
            .nth(self.current)
            .expect("Error while peeking in peek()...")
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            return '\0';
        }

        self.src
            .chars()
            .nth(self.current + 1)
            .expect("Error while peeking in peek_next()...")
    }

    fn handle_string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }

            self.advance();
        }

        if self.at_end() {
            eprintln!("Unterminated string on line {}", self.line);
            return;
        }

        self.advance();

        let value = self.src.as_str()[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::Stringy(value));
    }

    fn handle_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenType::Numeric(
            self.src.chars().as_str()[self.start..self.current]
                .parse::<f64>()
                .expect("Error while parsing number..."),
        ));
    }

    fn handle_identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        self.add_token(TokenType::Identifier);
    }
}
