use crate::statement::*;
use crate::BTreeMap;

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }

    pub fn compile(&self, scope: &mut CompileScope) {
        scope.vars.push(BTreeMap::new());
        self.statements.iter().for_each(|s| s.compile(scope));
        let last = scope.vars.pop().unwrap();
        (0..last.len()).for_each(|_| println!("Pop"));
        println!("-- end scope");
    }
}
