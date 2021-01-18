use crate::block::Block;
use crate::eval::atom::{FunctionCall, FunctionData};
use crate::statement::Declaration;
use crate::statement::Stmt;
use crate::Atom;
use crate::State;

pub fn eval_function_call(f: &FunctionCall, state: &mut State) -> Option<Atom> {
    let FunctionCall { name, args } = f;
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
