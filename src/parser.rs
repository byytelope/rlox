use crate::{
    ast::{Expr, Literal},
    error::{RError::RParseError, Result},
    token::{Token, TokenType},
};

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn binary_expr<F>(&mut self, ops: &[TokenType], mut right_fn: F) -> Result<Expr>
    where
        F: FnMut(&mut Self) -> Result<Expr>,
    {
        let mut expr = right_fn(self)?;

        while self.match_token(ops) {
            let operator = self.previous();
            let right = right_fn(self)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        self.binary_expr(&[TokenType::BangEqual, TokenType::EqualEqual], |p| {
            p.comparison()
        })
    }

    fn comparison(&mut self) -> Result<Expr> {
        self.binary_expr(
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
            |p| p.term(),
        )
    }

    fn term(&mut self) -> Result<Expr> {
        self.binary_expr(&[TokenType::Plus, TokenType::Minus], |p| p.factor())
    }

    fn factor(&mut self) -> Result<Expr> {
        self.binary_expr(&[TokenType::Star, TokenType::Slash], |p| p.unary())
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        let expr = match &self.peek().token_type {
            TokenType::False => Expr::Literal {
                value: Literal::Bool(false),
            },
            TokenType::True => Expr::Literal {
                value: Literal::Bool(true),
            },
            TokenType::Nil => Expr::Literal {
                value: Literal::None,
            },
            TokenType::Number(num) => Expr::Literal {
                value: Literal::Number(*num),
            },
            TokenType::Stringy(num) => Expr::Literal {
                value: Literal::Stringy(num.clone()),
            },
            TokenType::LeftParen => {
                let exp = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression")?;

                Expr::Grouping {
                    expression: Box::new(exp),
                }
            }
            _ => return Err(RParseError),
        };

        self.advance();

        Ok(expr)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if self.check_token(&token_type) {
            return Ok(self.advance().clone());
        }
        eprintln!("{}", message);

        Err(RParseError)
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn check_token(&self, token_type: &TokenType) -> bool {
        if self.at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        if token_types
            .iter()
            .any(|token_type| self.check_token(token_type))
        {
            self.advance();

            true
        } else {
            false
        }
    }
}
