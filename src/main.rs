use std::error::Error;
use std::io::{self, Read};

#[allow(dead_code)]
#[derive(Debug)]
enum TokenType {
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

struct Token {
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
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {}", self.ty, self.lexeme, self.literal)
    }
}

fn run(code: &String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn run_file(path: impl AsRef<std::path::Path> + std::fmt::Debug + std::clone::Clone) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    run(&file)?;
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        print!(">> ");
        stdin.read_to_string(&mut buffer)?;

        if buffer == "exit".to_string() {
            break;
        }

        run(&buffer)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        Ok(())
    } else if args.len() == 2 {
        run_file(args[1].clone())
    } else {
        run_prompt()
    }
}
