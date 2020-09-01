#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Slash,
    Star,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    Str,
    Number,
    And,
    Or,
    Struct,
    If,
    Else,
    Elif,
    True,
    False,
    Function,
    For,
    While,
    Print,
    EOF,
}

pub trait Literal: std::fmt::Display { }
impl Literal for String {}

pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Box<dyn Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, literal: Box<dyn Literal>, line: usize) -> Self {
        Token {
            ty,
            lexeme,
            literal,
            line,
        }
    }

    pub fn from_ty(ty: TokenType) -> Self {
        Token {
            ty,
            lexeme: "".to_string(),
            literal: Box::new("".to_string()),
            line: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.ty, self.lexeme, self.literal)
    }
}
