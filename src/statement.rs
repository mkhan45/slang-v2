use std::collections::HashMap;

use crate::{
    eval::{atom::Atom, eval_expr},
    parser::*,
};

#[derive(Default)]
pub struct State {
    vars: HashMap<String, Atom>,
}

impl State {
    fn declare(&mut self, dec: Declaration) {
        match self.vars.get(&dec.lhs) {
            Some(old_val) => {
                let new_val = eval_expr(&dec.rhs);
                if std::mem::discriminant(&old_val) == std::mem::discriminant(&&new_val) {
                    self.vars.insert(dec.lhs, new_val);
                }
            }
            None => {
                self.vars.insert(dec.lhs, eval_expr(&dec.rhs));
            }
        }
    }
}

#[derive(Debug)]
pub struct Declaration {
    lhs: String,
    rhs: S,
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(S),
    PrintStmt(S),
    Dec(Declaration),
}

impl Stmt {
    pub fn execute(self, state: &mut State) {
        match self {
            Stmt::ExprStmt(_expr) => {}
            Stmt::PrintStmt(expr) => println!("{}", eval_expr(&expr)),
            Stmt::Dec(dec) => state.declare(dec),
        }
    }
}
