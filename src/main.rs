#![feature(or_patterns)]
#![feature(type_alias_impl_trait)]

use std::error::Error;
use std::io;

mod scanner;
use scanner::token::*;
use scanner::*;

mod parser;
use parser::*;

mod eval;
use eval::eval_expr;

fn run(code: &str) -> Result<(), Box<dyn Error>> {
    let tokens = scan_tokens(code);
    if let Some(t) = tokens.iter().find(|t| t.ty == TokenType::Unknown) {
        Err(format!("Invalid {} on line {}", t.lexeme, t.line).into())
    } else {
        println!(
            "{:?}",
            tokens
                .clone()
                .iter()
                .map(|t| t.ty.clone())
                .collect::<Vec<TokenType>>()
        );
        let mut lexer = Lexer::new(tokens);
        let expr = parse_expr(&mut lexer);
        println!("{}", &expr);
        println!("{}", eval_expr(&expr));
        Ok(())
    }
}

fn run_file(
    path: impl AsRef<std::path::Path> + std::fmt::Debug + std::clone::Clone,
) -> Result<(), Box<dyn Error>> {
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

        if buffer == *"exit".to_string() {
            break;
        }

        run(&buffer)?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    match args.len() {
        0 | 1 => run_prompt(),
        2 => run_file(args[1].clone()),
        _ => {
            println!("Usage: rlox [script]");
            Ok(())
        }
    }
}
