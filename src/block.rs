use crate::statement::*;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn compile(&self, scope: &mut CompileScope) {
        self.statements.iter().for_each(|s| s.compile(scope));
    }
}
