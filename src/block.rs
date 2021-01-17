use crate::{
    eval::{atom::Atom, eval_expr},
    statement::*,
};

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn execute(&mut self, scope: &mut State) -> Option<Atom> {
        let mut res = None;
        self.statements.drain(..).for_each(|stmt| {
            res = stmt.execute(scope);
        });
        res
    }
}
