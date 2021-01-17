use crate::{eval::atom::Atom, statement::*};

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn execute(&mut self, state: &mut State) -> Option<Atom> {
        state.scopes.push(Scope::default());
        let mut res = None;
        self.statements.iter().cloned().for_each(|stmt| {
            res = stmt.execute(state);
        });
        state.scopes.pop();
        res
    }
}
