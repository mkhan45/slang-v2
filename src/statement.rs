use std::collections::BTreeMap;

use crate::{block::Block, eval::atom::Atom, parser::*};

#[derive(Default, Debug, Clone)]
pub struct Scope {
    pub vars: BTreeMap<String, Atom>,
}

#[derive(Default, Debug, Clone)]
pub struct CompileScope {
    pub vars: Vec<BTreeMap<String, usize>>,
    pub label_count: usize,
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
    pub fn compile(&self, scope: &mut CompileScope) {
        use Stmt::*;

        match self {
            ExprStmt(s) => {
                println!("-- start expr stmt");
                s.compile(scope);
                println!("-- end expr stmt");
            }
            PrintStmt(s) => {
                println!("-- start print stmt");
                s.compile(scope);
                println!("Print");
                println!("Push 10");
                println!("PrintC");
                println!("Pop");
                println!("Pop");
                println!("-- end print stmt");
            }
            Dec(Declaration {
                lhs,
                rhs,
                alias,
                plus_or_minus,
            }) => {
                println!("-- start declaration of {}", lhs);
                if *alias {
                    let top_scope = scope.vars.last_mut().unwrap();
                    let len = top_scope.len();
                    top_scope.insert(lhs.to_string(), len);
                    rhs.compile(scope);
                } else {
                    let mut full_len: usize = scope.vars.iter().map(|s| s.len()).sum();
                    for s in scope.vars.iter_mut().rev() {
                        if s.contains_key(lhs) {
                            let i = full_len - s.len() + *s.get(lhs).unwrap();

                            if plus_or_minus.is_some() {
                                println!("Get {}", i);
                                rhs.compile(scope);
                                if plus_or_minus.unwrap() {
                                    println!("Add");
                                } else {
                                    println!("Sub");
                                }
                                println!("Set {}", i);
                                println!("Pop");
                            } else {
                                rhs.compile(scope);
                                println!("Set {}", i);
                                println!("Pop");
                            }

                            break;
                        }
                        full_len -= s.len();
                    }
                }
                println!("-- end declaration of {}", lhs);
            }
            IfStmt(If {
                cond,
                then_block,
                else_block,
            }) => {
                println!("-- start if block");
                cond.compile(scope);
                println!("JE {}", scope.label_count);
                println!("Pop");
                then_block.compile(scope);
                println!("Jump {}", scope.label_count + 1);
                println!("label {}", scope.label_count);
                else_block.compile(scope);
                println!("label {}", scope.label_count + 1);
                scope.label_count += 2;
                println!("-- end if block");
            }
            WhileStmt(While { cond, loop_block }) => {
                println!("-- start while block");
                println!("label {}", scope.label_count + 1);
                cond.compile(scope);
                println!("JE {}", scope.label_count + 2);
                println!("Pop");
                loop_block.compile(scope);
                println!("Jump {}", scope.label_count + 1);
                println!("label {}", scope.label_count + 2);
                scope.label_count += 2;
                println!("-- end while block");
            }
            Block(b) => {
                b.compile(scope);
            }
            Break => {}
        }
        println!();
    }
}
