use crate::Error;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn evaluate(&self) -> Result<String, Error> {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Grouping { expression } => todo!(),
            Expr::Literal { value } => todo!(),
            Expr::Unary { operator, right } => todo!(),
        }
    }

    pub fn print(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(&operator.token_type.to_string(), vec![left, right]),
            Expr::Grouping { expression } => self.parenthesize("group", vec![expression]),
            Expr::Literal { value } => match value {
                Literal::Stringy(literal) => literal.to_string(),
                Literal::Number(literal) => literal.to_string(),
                Literal::Bool(literal) => literal.to_string(),
                Literal::None => "nil".to_string(),
            },
            Expr::Unary { operator, right } => {
                self.parenthesize(&operator.token_type.to_string(), vec![right])
            }
        }
    }

    fn parenthesize(&self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut buf = format!("({}", name);

        exprs.iter().for_each(|expr| {
            buf.push(' ');
            buf.push_str(expr.print().as_str())
        });

        buf.push(')');

        buf
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Number(f64),
    Stringy(String),
    None,
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn check_print() {
        let lop = Token::new(TokenType::Minus, 0);
        let op = Token::new(TokenType::Minus, 0);

        let expr = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: lop,
                right: Box::new(Expr::Literal {
                    value: Literal::Number(123.0),
                }),
            }),
            operator: op,
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };

        assert_eq!(expr.print(), "(* (- 123) (group 45.67))")
    }
}
