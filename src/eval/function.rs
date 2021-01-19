use crate::eval::atom::{FunctionCall, FunctionData};
use crate::statement::Declaration;
use crate::statement::Stmt;
use crate::Atom;
use crate::State;
use crate::{block::Block, parser::S};

use super::eval_expr;

pub fn eval_function_call(f: &FunctionCall, state: &mut State) -> Option<Atom> {
    let FunctionCall { name, args } = f;

    if name == "push" {
        let res = array_push(&args[0], &args[1], state);
        return Some(res);
    }

    if name == "len" {
        let res = array_len(&args[0], state);
        return Some(res);
    }

    if let Some(Atom::Function(fn_data)) = state.get_variable(name) {
        let FunctionData {
            arg_names,
            fn_block,
        } = fn_data;
        let mut full_block_statements: Vec<Stmt> =
            Vec::with_capacity(args.len() + fn_block.statements.len());
        args.iter()
            .zip(arg_names.iter())
            .for_each(|(arg_val, arg_name)| {
                full_block_statements.push(Stmt::Dec(Declaration {
                    lhs: arg_name.to_string(),
                    rhs: arg_val.clone(),
                    alias: true,
                    plus_or_minus: None,
                }))
            });

        full_block_statements.append(&mut fn_block.clone().statements);

        let mut full_block = Block::new(full_block_statements);
        full_block.execute(state)
    } else {
        panic!("Function {} is undefined", name);
    }
}

fn array_push(a: &S, e: &S, state: &mut State) -> Atom {
    match (a, e) {
        (S::Atom(Atom::Identifier(arr_name)), s) => {
            let new_val = eval_expr(s, state);
            if let Some(Atom::Array(a)) = state.get_variable(&arr_name) {
                // this could be sped up a lot by adding a get mut function
                // to State
                let mut new_arr = a.clone();
                new_arr.push(S::Atom(new_val.clone()));
                state.declare(Declaration {
                    lhs: arr_name.to_string(),
                    rhs: S::Atom(Atom::Array(new_arr)),
                    alias: false,
                    plus_or_minus: None,
                });
                new_val
            } else {
                panic!("{} is not an array or undefined", arr_name);
            }
        }
        _ => panic!("Can't push"),
    }
}

fn array_len(a: &S, state: &mut State) -> Atom {
    if let S::Atom(Atom::Identifier(arr_name)) = a {
        if let Some(Atom::Array(a)) = state.get_variable(&arr_name) {
            return Atom::Num(a.len() as f64);
        }
    }
    panic!("{:?} is not an array", a)
}
