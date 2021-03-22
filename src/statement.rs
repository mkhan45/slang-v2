use std::collections::BTreeMap;

use crate::{block::Block, eval::atom::Atom, parser::*};

#[derive(Default, Debug, Clone)]
pub struct Scope {
    pub vars: BTreeMap<String, Atom>,
}

#[derive(Default, Debug, Clone)]
pub struct CompileScope {
    pub vars: BTreeMap<String, usize>,
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

        println!();
        match self {
            ExprStmt(s) => {
                s.compile(scope);
            }
            PrintStmt(s) => {
                s.compile(scope);
                println!("Print");
                println!("Push 10");
                println!("PrintC");
                println!("Pop");
                println!("Pop");
            }
            Dec(Declaration {
                lhs,
                rhs,
                alias,
                plus_or_minus,
            }) => {
                if scope.vars.keys().any(|k| k == lhs) {
                    if !alias && plus_or_minus.is_some() {
                        let i = *scope.vars.get(lhs).unwrap();
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
                        let i = *scope.vars.get(lhs).unwrap();
                        rhs.compile(scope);
                        println!("Set {}", i);
                        println!("Pop");
                    }
                } else {
                    scope.vars.insert(lhs.to_string(), scope.vars.len());
                    rhs.compile(scope);
                }
            }
            IfStmt(If {
                cond,
                then_block,
                else_block,
            }) => {
                cond.compile(scope);
                println!("JE {}", scope.label_count);
                println!("Pop");
                then_block.compile(scope);
                println!("Jump {}", scope.label_count + 1);
                println!("label {}", scope.label_count);
                else_block.compile(scope);
                println!("label {}", scope.label_count + 1);
                scope.label_count += 2;
            }
            WhileStmt(While { cond, loop_block }) => {
                println!("label {}", scope.label_count + 1);
                cond.compile(scope);
                println!("JE {}", scope.label_count + 2);
                loop_block.compile(scope);
                println!("Jump {}", scope.label_count + 1);
                println!("label {}", scope.label_count + 2);
                scope.label_count += 2;
            }
            Block(crate::block::Block { statements }) => {
                statements.iter().for_each(|s| s.compile(scope));
            }
            Break => {}
        }
        println!();
    }
}

#[cfg(test)]
mod stmt_tests {
    use crate::run_file;
    use crate::Atom;
    use crate::State;

    macro_rules! test_files {
        () => {};
        ( $fn_name:ident, $file:expr => $expected:expr; $($tail:tt)* ) => {
            #[test]
            fn $fn_name() {
                let mut top_state = State::default();
                let output = run_file(format!("test_files/{}", $file), &mut top_state).unwrap();
                assert_eq!(output, $expected);
            }

            test_files!($($tail)*);
        };
        ( $fn_name:ident, $file:expr; $($tail:tt)* ) => {
            #[test]
            #[should_panic]
            fn $fn_name() {
                let mut top_state = State::default();
                run_file(format!("test_files/{}", $file), &mut top_state).unwrap();
            }

            test_files!($($tail)*);
        };
    }

    test_files!(
        basic1, "basic1.slang" => Some(Atom::Int(20));
        basic2, "basic2.slang" => Some(Atom::Int(5));
        if1, "if.slang" => Some(Atom::Str("hello".to_string()));
        if2, "else.slang" => Some(Atom::Str("goodbye".to_string()));
        scope_modify, "scope_modify.slang" => Some(Atom::Int(2));
        while1, "while1.slang" => Some(Atom::Int(10));
        for1, "for1.slang" => Some(Atom::Int(1053));
        fn1, "fn1.slang" => Some(Atom::Int(120));
        euler01, "project_euler_01.slang" => Some(Atom::Int(233168));
        euler02, "project_euler_02.slang" => Some(Atom::Int(4613732));
        scoped_loop, "scoped_loop.slang" => Some(Atom::Int(45));
        loop_break, "loop_break.slang" => Some(Atom::Int(5));
        nested_loop_break, "nested_loop_break.slang" => Some(Atom::Int(25));
        recur1, "recursion01.slang" => Some(Atom::Int(987));
        error1, "error1.slang";
        scope_typecheck, "scope_typecheck.slang";
    );
}
