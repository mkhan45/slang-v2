#![feature(or_patterns)]
#![feature(type_alias_impl_trait)]

use std::error::Error;
// use std::io::{self, Read};
use std::io;

mod token;
use crate::token::*;

fn run(code: &String) -> Result<(), Box<dyn Error>> {
    let tokens = scan_tokens(code);
    if let Some(t) = tokens.iter().find(|t| t.ty == TokenType::Unknown) {
        Err(format!("Invalid {} on line {}", t.lexeme, t.line).into())
    } else {
        tokens.iter().for_each(|t| println!("{:?}", t.ty));
        Ok(())
    }
}

fn run_file(path: impl AsRef<std::path::Path> + std::fmt::Debug + std::clone::Clone) -> Result<(), Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    run(&file)?;
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        println!(">> ");
        buffer.clear();
        stdin.read_line(&mut buffer)?;

        if buffer == "exit".to_string() {
            break;
        }

        run(&buffer)?;
    }

    Ok(())
}

type Scanner<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn scan_string(source: &mut Scanner, line: usize) -> Token {
    let res = source.take_while(|&c| c != '\"').collect::<String>();
    if source.next().is_some() {
        let token = Token::new(TokenType::Str, res.clone(), Box::new(res), line);
        token
    } else {
        Token::new(TokenType::Unknown, res, Box::new("".to_string()), line)
    }
}

fn scan_number(source: &mut Scanner, line: usize) -> Token {
    let res = source.take_while(|&c| c.is_numeric() || c == '.').collect::<String>();
    if let Ok(n) = res.clone().parse::<f32>() {
        Token::new(TokenType::Number, res.clone(), Box::new(n), line)
    } else {
        Token::new(TokenType::Unknown, res.clone(), Box::new(res), line)
    }
}

fn skip_comment(source: &mut Scanner) {
    while let Some(c) = source.next() {
        if c == '\n' {
            break;
        }
    }
}

fn scan_tokens(source: &String) -> Vec<Token> {
    let mut char_iter = source.chars().peekable();
    let mut line = 1;

    let next_token = || {
        let c = char_iter.next();
        let peek = char_iter.peek();
        match (c, peek) {
            (Some(' '|'\t'), _) => Some(Token::from_ty(TokenType::WhiteSpace)),
            (Some('('), _) => Some(Token::from_ty(TokenType::LParen)),
            (Some(')'), _) => Some(Token::from_ty(TokenType::RParen)),
            (Some('{'), _) => Some(Token::from_ty(TokenType::LBrace)),
            (Some('}'), _) => Some(Token::from_ty(TokenType::RBrace)),
            (Some(','), _) => Some(Token::from_ty(TokenType::Comma)),
            (Some('+'), _) => Some(Token::from_ty(TokenType::Plus)),
            (Some('-'), _) => Some(Token::from_ty(TokenType::Minus)),
            (Some('*'), _) => Some(Token::from_ty(TokenType::Star)),
            (Some('#'), _) => {
                skip_comment(&mut char_iter);
                Some(Token::from_ty(TokenType::Hash))
            },
            (Some('\n'|'\r'), _) => {
                line += 1;
                Some(Token::from_ty(TokenType::WhiteSpace))
            },
            (Some('!'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::BangEqual))
            },
            (Some('!'), _) => Some(Token::from_ty(TokenType::Bang)),
            (Some('<'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::LessEqual))
            },
            (Some('<'), _) => Some(Token::from_ty(TokenType::Less)),
            (Some('>'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::GreaterEqual))
            },
            (Some('>'), _) => Some(Token::from_ty(TokenType::Greater)),
            (Some('='), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::Equal))
            },
            (Some('='), _) => Some(Token::from_ty(TokenType::Assign)),
            (Some('\"'), _) => Some(scan_string(&mut char_iter, line)),
            (Some(c), _n_opt) => {
                if c.is_numeric() {
                    Some(scan_number(&mut char_iter, line))
                } else {
                    Some(Token::new(TokenType::Unknown, c.to_string(), Box::new(c.to_string()), line))
                }
            }
            (_, _) => None,
        }
    };

    let token_iter = std::iter::from_fn(next_token);

    token_iter.filter(|t| ![TokenType::WhiteSpace, TokenType::Hash].contains(&t.ty)).collect::<Vec<Token>>()
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
