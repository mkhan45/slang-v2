#![feature(or_patterns)]
#![feature(type_alias_impl_trait)]
#![feature(bindings_after_at)]

use crate::eval::atom::Atom;
use crate::statement::CompileScope;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::{collections::BTreeMap, error::Error};

mod scanner;
use scanner::token::*;
use scanner::*;

mod parser;
use parser::*;

mod eval;
use statement::State;

mod statement;

mod block;

fn run(code: &str, state: &mut State, unscoped: bool) -> Result<Option<Atom>, Box<dyn Error>> {
    let tokens = scan_tokens(code);
    if let Some(t) = tokens.iter().find(|t| t.ty == TokenType::Unknown) {
        Err(format!("Invalid {:?} ({}) on line {}", t.ty, t.lexeme, t.line).into())
    } else {
        // println!(
        //     "{:?}",
        //     tokens
        //         .clone()
        //         .iter()
        //         .map(|t| t.ty.clone())
        //         .collect::<Vec<TokenType>>()
        // );
        let mut lexer = Lexer::new(tokens);

        let mut main_block = parse_block(&mut lexer);

        let mut scope = CompileScope {
            vars: BTreeMap::new(),
            label_count: 0,
        };
        main_block.compile(&mut scope);

        // let res = if unscoped {
        //     main_block.execute_unscoped(state)
        // } else {
        //     main_block.execute_unscoped(state)
        // };

        // Ok(res)
        Ok(None)
    }
}

fn run_file(
    path: impl AsRef<std::path::Path> + std::fmt::Debug + std::clone::Clone,
    state: &mut State,
) -> Result<Option<Atom>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    let res = run(&file, state, false)?;
    if let Some(ref a) = res {
        println!("{}", a);
    }

    Ok(res)
}

fn run_prompt(state: &mut State) -> Result<Option<Atom>, Box<dyn Error>> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = Vec::new();

    loop {
        print!("Slang |>\n");
        stdout.flush()?;
        buffer.clear();

        // stupid hack, REPL goes until ~
        stdin.lock().read_until(b'~', &mut buffer)?;

        let buffer_str = std::str::from_utf8(&buffer).unwrap();

        if buffer_str == "exit" {
            break;
        }

        if let Some(a) = run(&buffer_str[..buffer.len() - 2], state, true)? {
            println!("{}", a);
        }
    }

    // TODO: make this return the last expr
    Ok(None)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();

    let mut top_state = State::default();

    {
        use crate::block::Block;
        use crate::eval::atom::FunctionData;
        use crate::statement::{Declaration, Stmt};
        top_state.declare(Declaration {
            lhs: "mul".to_string(),
            rhs: S::Atom(Atom::Function(FunctionData {
                arg_names: vec!["x".to_string(), "y".to_string()],
                fn_block: Block::new(vec![Stmt::ExprStmt(S::Cons(
                    Op::Multiply,
                    vec![
                        S::Atom(Atom::Identifier("x".to_string())),
                        S::Atom(Atom::Identifier("y".to_string())),
                    ],
                ))]),
            })),
            alias: true,
            plus_or_minus: None,
        });
    }

    match args.len() {
        0 | 1 => run_prompt(&mut top_state),
        2 => run_file(args[1].clone(), &mut top_state),
        _ => {
            println!("Usage: rlox [script]");
            Err("bad input".into())
        }
    }?;

    Ok(())
}
