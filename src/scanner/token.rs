use crate::eval::atom::Atom;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Let,
    WhiteSpace,
    NewLine,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Plus,
    PlusAssign,
    Minus,
    MinusAssign,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    Assign,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    Literal(Atom),
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
    Hash,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, line: usize) -> Self {
        Token { ty, lexeme, line }
    }

    pub fn unknown(line: usize) -> Self {
        Token {
            ty: TokenType::Unknown,
            lexeme: "".to_string(),
            line,
        }
    }

    pub fn from_ty(ty: TokenType) -> Self {
        Token {
            ty,
            lexeme: "".to_string(),
            line: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.ty, self.lexeme)
    }
}
