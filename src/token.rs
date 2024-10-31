use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: impl ToString, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            line,
        }
    }
}
