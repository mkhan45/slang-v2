use std::collections::HashMap;

use crate::{
    block::Block,
    eval::{atom::Atom, eval_expr},
    parser::*,
};

#[derive(Default, Debug)]
pub struct State {
    pub vars: HashMap<String, Atom>,
}

impl State {
    fn declare(&mut self, dec: Declaration) {
        let disc = {
            let var = self.vars.get(&dec.lhs);
            var.map(std::mem::discriminant)
        };

        match (disc, dec.alias) {
            (Some(d), alias) => {
                let new_val = eval_expr(&dec.rhs, self);
                if d == std::mem::discriminant(&&new_val) || alias {
                    self.vars.insert(dec.lhs, new_val);
                } else {
                    panic!(
                        "Mismatched types for {}, can't assign {:?} to {:?}",
                        dec.lhs,
                        new_val,
                        self.vars.get(&dec.lhs).unwrap()
                    );
                }
            }
            (None, true) => {
                let new_val = eval_expr(&dec.rhs, self);
                self.vars.insert(dec.lhs, new_val);
            }
            (None, false) => {
                panic!("Uninitialized variable {}", dec.lhs)
            }
        }
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub lhs: String,
    pub rhs: S,
    pub alias: bool,
}

#[derive(Debug)]
pub struct If {
    pub cond: S,
    pub then_block: Block,
    pub else_block: Block,
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(S),
    PrintStmt(S),
    Dec(Declaration),
    IfStmt(If),
}

impl Stmt {
    pub fn execute(self, state: &mut State) -> Option<Atom> {
        match self {
            Stmt::ExprStmt(expr) => Some(eval_expr(&expr, state)),
            Stmt::PrintStmt(expr) => {
                println!("{}", eval_expr(&expr, state));
                None
            }
            Stmt::Dec(dec) => {
                state.declare(dec);
                None
            }
            Stmt::IfStmt(if_data) => {
                let If {
                    cond,
                    mut then_block,
                    mut else_block,
                } = if_data;
                if eval_expr(&cond, state) == Atom::Bool(true) {
                    then_block.execute(state);
                } else {
                    else_block.execute(state);
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod stmt_tests {
    use crate::run_file;
    use crate::Atom;
    use crate::State;

    macro_rules! test_files {
        () => {};
        ( $fn_name:ident, $file:expr => $expected:expr; $($tail:tt)* ) => {
            #[test]
            fn $fn_name() {
                let mut top_state = State::default();
                let output = run_file(format!("test_files/{}", $file), &mut top_state).unwrap();
                assert_eq!(output, $expected);
            }

            test_files!($($tail)*);
        };
        ( $fn_name:ident, $file:expr; $($tail:tt)* ) => {
            #[test]
            #[should_panic]
            fn $fn_name() {
                let mut top_state = State::default();
                run_file(format!("test_files/{}", $file), &mut top_state).unwrap();
            }

            test_files!($($tail)*);
        };
    }

    test_files!(
        basic1, "basic1.slang" => Some(Atom::Num(20.0));
        basic2, "basic2.slang" => Some(Atom::Num(5.0));
        error1, "error1.slang";
    );
}
