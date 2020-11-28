use std::collections::HashMap;

use crate::{
    eval::{atom::Atom, eval_expr},
    parser::*,
};

#[derive(Default)]
pub struct State {
    pub vars: HashMap<String, Atom>,
}

impl State {
    fn declare(&mut self, dec: Declaration) {
        let disc = {
            let var = self.vars.get(&dec.lhs);
            var.map(std::mem::discriminant)
        };

        match disc {
            Some(d) => {
                let new_val = eval_expr(&dec.rhs, self);
                if d == std::mem::discriminant(&&new_val) {
                    self.vars.insert(dec.lhs, new_val);
                }
            }
            None => {
                let new_val = eval_expr(&dec.rhs, self);
                self.vars.insert(dec.lhs, new_val);
            }
        }
    }
}

#[derive(Debug)]
pub struct Declaration {
    pub lhs: String,
    pub rhs: S,
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
            Stmt::ExprStmt(expr) => {
                eval_expr(&expr, state);
            }
            Stmt::PrintStmt(expr) => println!("{}", eval_expr(&expr, state)),
            Stmt::Dec(dec) => state.declare(dec),
        }
    }
}
