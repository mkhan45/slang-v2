use std::collections::HashMap;

use crate::{
    block::Block,
    eval::{atom::Atom, eval_expr},
    parser::*,
};

#[derive(Debug, Clone)]
pub struct State {
    pub scopes: Vec<Scope>,
}

impl Default for State {
    fn default() -> Self {
        State {
            scopes: vec![Scope::default()],
        }
    }
}

impl State {
    pub fn get_variable(&self, var: &str) -> Option<&Atom> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.vars.get(var))
    }

    fn modify_variable(&mut self, var: &str, val: Atom) {
        for scope in &mut self.scopes {
            if scope.vars.contains_key(var) {
                scope.vars.insert(var.to_string(), val);
                break;
            }
        }
    }

    pub fn declare(&mut self, dec: Declaration) {
        let (val, disc) = {
            let var = self.get_variable(&dec.lhs);
            (var.cloned(), var.map(std::mem::discriminant))
        };

        match (disc, dec.alias) {
            (_, true) => {
                let new_val = eval_expr(&dec.rhs, self);
                self.scopes
                    .last_mut()
                    .unwrap()
                    .vars
                    .insert(dec.lhs, new_val);
            }
            (Some(d), false) => {
                let rhs_val = eval_expr(&dec.rhs, self);
                if d == std::mem::discriminant(&&rhs_val) {
                    let new_val = match dec.plus_or_minus {
                        Some(true) => val.unwrap() + rhs_val,
                        Some(false) => val.unwrap() - rhs_val,
                        None => rhs_val,
                    };

                    self.modify_variable(&dec.lhs, new_val);
                } else {
                    panic!("Cannot assign {:?} to {:?}", rhs_val, val);
                }
            }
            (None, false) => {
                panic!("Uninitialized variable {}", dec.lhs)
            }
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Scope {
    pub vars: HashMap<String, Atom>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub lhs: String,
    pub rhs: S,
    pub alias: bool,
    pub plus_or_minus: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: S,
    pub then_block: Block,
    pub else_block: Block,
}

#[derive(Debug, Clone)]
pub struct While {
    pub cond: S,
    pub loop_block: Block,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(S),
    PrintStmt(S),
    Dec(Declaration),
    IfStmt(If),
    WhileStmt(While),
    Block(Block),
    Break,
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
                    then_block.execute(state)
                } else {
                    else_block.execute(state)
                }
            }
            Stmt::WhileStmt(while_data) => {
                let While {
                    cond,
                    mut loop_block,
                } = while_data;

                while eval_expr(&cond, state) == Atom::Bool(true) {
                    let res = loop_block.execute(state);
                    if Some(Atom::Break) == res {
                        break;
                    }
                }
                None
            }
            Stmt::Block(mut b) => b.execute(state),
            Stmt::Break => Some(Atom::Break),
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
        if1, "if.slang" => Some(Atom::Str("hello".to_string()));
        if2, "else.slang" => Some(Atom::Str("goodbye".to_string()));
        scope_modify, "scope_modify.slang" => Some(Atom::Num(2.0));
        while1, "while1.slang" => Some(Atom::Num(10.0));
        for1, "for1.slang" => Some(Atom::Num(1053.0));
        fn1, "fn1.slang" => Some(Atom::Num(120.0));
        euler01, "project_euler_01.slang" => Some(Atom::Num(233168.0));
        euler02, "project_euler_02.slang" => Some(Atom::Num(4613732.0));
        recur1, "recursion01.slang" => Some(Atom::Num(987.0));
        error1, "error1.slang";
        scope_typecheck, "scope_typecheck.slang";
    );
}
