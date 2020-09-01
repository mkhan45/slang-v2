use std::error::Error;
use std::io::{self, Read};

mod token;
use crate::token::*;

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

struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    fn scan_tokens(mut self) {
        let mut char_iter = self.source.chars().peekable();

        let mut next_token = || {
            let c = char_iter.next();
            let peek = char_iter.peek();
            match (c, peek) {
                (Some('('), _) => Token::from_ty(TokenType::LParen),
                (Some(')'), _) => Token::from_ty(TokenType::RParen),
                (Some('{'), _) => Token::from_ty(TokenType::LBrace),
                (Some('}'), _) => Token::from_ty(TokenType::RBrace),
                (Some(','), _) => Token::from_ty(TokenType::Comma),
                (Some('+'), _) => Token::from_ty(TokenType::Plus),
                (Some('-'), _) => Token::from_ty(TokenType::Minus),
                (Some('*'), _) => Token::from_ty(TokenType::Star),
                _ => Token::from_ty(TokenType::EOF),
            }
        };

        loop {
            let token = next_token();
            if token.ty == TokenType::EOF {
                break;
            } else {
                self.tokens.push(token);
            }
        }
    }
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
