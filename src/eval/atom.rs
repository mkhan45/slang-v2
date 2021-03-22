use crate::parser::S;
use crate::{block::Block, statement::CompileScope};

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Int(isize),
    Identifier(String),
}

impl Atom {
    pub fn compile(&self, scope: &CompileScope) {
        use Atom::*;

        match self {
            Int(i) => println!("Push {}", i),
            Identifier(n) => {
                let i = scope.vars.get(n).unwrap();
                println!("Get {}", i);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<S>,
}

impl PartialEq for FunctionCall {
    fn eq(&self, _rhs: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub arg_names: Vec<String>,
    pub fn_block: Block,
}

impl PartialEq for FunctionData {
    fn eq(&self, _rhs: &Self) -> bool {
        //TODO: idk but this should probably return true sometimes
        false
    }
}
