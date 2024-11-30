use std::{collections::HashMap, fmt::Display, sync::LazyLock};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    Stringy(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Single-character tokens.
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),

            // One or two character tokens.
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),

            // Literals.
            TokenType::Identifier(name) => write!(f, "Identifier({})", name),
            TokenType::Stringy(value) => write!(f, "String(\"{}\")", value),
            TokenType::Number(num) => write!(f, "Number({})", num),

            // Keywords.
            TokenType::And => write!(f, "and"),
            TokenType::Class => write!(f, "class"),
            TokenType::Else => write!(f, "else"),
            TokenType::False => write!(f, "false"),
            TokenType::Fun => write!(f, "fun"),
            TokenType::For => write!(f, "for"),
            TokenType::If => write!(f, "if"),
            TokenType::Nil => write!(f, "nil"),
            TokenType::Or => write!(f, "or"),
            TokenType::Print => write!(f, "print"),
            TokenType::Return => write!(f, "return"),
            TokenType::Super => write!(f, "super"),
            TokenType::This => write!(f, "this"),
            TokenType::True => write!(f, "true"),
            TokenType::Var => write!(f, "var"),
            TokenType::While => write!(f, "while"),

            // End of file.
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

impl TokenType {
    pub fn get_keyword(keyword: &str) -> Option<&Self> {
        KEYWORDS.get(keyword)
    }
}

// Skip inner type cmp
impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (TokenType::LeftParen, TokenType::LeftParen)
                | (TokenType::RightParen, TokenType::RightParen)
                | (TokenType::LeftBrace, TokenType::LeftBrace)
                | (TokenType::RightBrace, TokenType::RightBrace)
                | (TokenType::Comma, TokenType::Comma)
                | (TokenType::Dot, TokenType::Dot)
                | (TokenType::Minus, TokenType::Minus)
                | (TokenType::Plus, TokenType::Plus)
                | (TokenType::Semicolon, TokenType::Semicolon)
                | (TokenType::Slash, TokenType::Slash)
                | (TokenType::Star, TokenType::Star)
                | (TokenType::Bang, TokenType::Bang)
                | (TokenType::BangEqual, TokenType::BangEqual)
                | (TokenType::Equal, TokenType::Equal)
                | (TokenType::EqualEqual, TokenType::EqualEqual)
                | (TokenType::Greater, TokenType::Greater)
                | (TokenType::GreaterEqual, TokenType::GreaterEqual)
                | (TokenType::Less, TokenType::Less)
                | (TokenType::LessEqual, TokenType::LessEqual)
                | (TokenType::And, TokenType::And)
                | (TokenType::Class, TokenType::Class)
                | (TokenType::Else, TokenType::Else)
                | (TokenType::False, TokenType::False)
                | (TokenType::Fun, TokenType::Fun)
                | (TokenType::For, TokenType::For)
                | (TokenType::If, TokenType::If)
                | (TokenType::Nil, TokenType::Nil)
                | (TokenType::Or, TokenType::Or)
                | (TokenType::Print, TokenType::Print)
                | (TokenType::Return, TokenType::Return)
                | (TokenType::Super, TokenType::Super)
                | (TokenType::This, TokenType::This)
                | (TokenType::True, TokenType::True)
                | (TokenType::Var, TokenType::Var)
                | (TokenType::While, TokenType::While)
                | (TokenType::Eof, TokenType::Eof)
                | (TokenType::Identifier(_), TokenType::Identifier(_))
                | (TokenType::Stringy(_), TokenType::Stringy(_))
                | (TokenType::Number(_), TokenType::Number(_))
        )
    }
}

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    [
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]
    .iter()
    .cloned()
    .collect()
});
