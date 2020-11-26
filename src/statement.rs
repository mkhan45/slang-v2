use std::collections::HashMap;

use crate::{
    eval::{atom::Atom, eval_expr},
    parser::*,
};

pub struct State {
    vars: HashMap<String, Atom>,
}

impl State {
    fn add_declaration(&mut self, dec: Declaration) {
        self.vars.insert(dec.lhs, eval_expr(&dec.rhs));
    }
}

pub struct Declaration {
    lhs: String,
    rhs: S,
}

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
            Stmt::Dec(dec) => state.add_declaration(dec),
        }
    }
}
