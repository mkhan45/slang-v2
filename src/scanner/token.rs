#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum TokenType {
    Str,
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
    Hash,
    Unknown,
}

#[derive(Debug)]
pub enum Atom {
    Str(String),
    Num(f32),
}

pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Option<Atom>,
    pub line: usize,
}

impl Token {
    pub fn new(ty: TokenType, lexeme: String, literal: Option<Atom>, line: usize) -> Self {
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
            literal: None,
            line: 0,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.ty, self.lexeme, self.literal)
    }
}
