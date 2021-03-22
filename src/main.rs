#![feature(or_patterns)]
#![feature(type_alias_impl_trait)]
#![feature(bindings_after_at)]

use crate::eval::atom::Atom;
use crate::statement::CompileScope;
use std::{collections::BTreeMap, error::Error};

mod scanner;
use scanner::token::*;
use scanner::*;

mod parser;
use parser::*;

mod eval;

mod statement;

mod block;

fn run(code: &str) -> Result<Option<Atom>, Box<dyn Error>> {
    let tokens = scan_tokens(code);
    if let Some(t) = tokens.iter().find(|t| t.ty == TokenType::Unknown) {
        Err(format!("Invalid {:?} ({}) on line {}", t.ty, t.lexeme, t.line).into())
    } else {
        let mut lexer = Lexer::new(tokens);

        let main_block = parse_block(&mut lexer);

        let mut scope = CompileScope {
            vars: BTreeMap::new(),
            label_count: 0,
        };
        main_block.compile(&mut scope);

        Ok(None)
    }
}

fn run_file(
    path: impl AsRef<std::path::Path> + std::fmt::Debug + std::clone::Clone,
) -> Result<Option<Atom>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    let res = run(&file)?;

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    run_file(args[1].clone())?;

    Ok(())
}
