use crate::{eval::atom::Atom, statement::*};

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn execute_unscoped(&mut self, state: &mut State) -> Option<Atom> {
        let mut res = None;

        for stmt in self.statements.iter().cloned() {
            res = stmt.execute(state);
            if matches!(res, Some(Atom::Break)) {
                break;
            }
        }

        res
    }

    pub fn execute(&mut self, state: &mut State) -> Option<Atom> {
        state.scopes.push(Scope::default());
        let res = self.execute_unscoped(state);
        state.scopes.pop();
        res
    }
}
