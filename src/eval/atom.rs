use crate::{block::Block, statement::CompileScope};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use crate::parser::S;

#[derive(Debug, Clone)]
pub enum Atom {
    Str(String),
    Float(f64),
    Int(isize),
    Bool(bool),
    Identifier(String),
    FnCall(FunctionCall),
    Function(FunctionData),
    Array(Vec<S>),
    Break,
}

impl PartialEq for Atom {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Atom::Str(a), Atom::Str(b)) => a == b,
            (Atom::Float(a), Atom::Float(b)) => a == b,
            (Atom::Int(a), Atom::Int(b)) => a == b,
            (Atom::Bool(a), Atom::Bool(b)) => a == b,
            (Atom::Identifier(a), Atom::Identifier(b)) => a == b,
            _ => false,
        }
    }
}

impl Add for Atom {
    type Output = Atom;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Atom::Str(a), Atom::Str(b)) => Atom::Str(format!("{}{}", a, b)),
            (Atom::Str(s), Atom::Float(f)) => Atom::Str(format!("{}{}", s, f)),
            (Atom::Float(f), Atom::Str(s)) => Atom::Str(format!("{}{}", f, s)),
            (Atom::Str(s), Atom::Int(i)) => Atom::Str(format!("{}{}", s, i)),
            (Atom::Int(i), Atom::Str(s)) => Atom::Str(format!("{}{}", i, s)),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a + b),
            (Atom::Int(a), Atom::Float(b)) => Atom::Float(*a as f64 + b),
            (Atom::Float(a), Atom::Int(b)) => Atom::Float(a + *b as f64),
            (Atom::Int(a), Atom::Int(b)) => Atom::Int(a + b),
            _ => panic!("Add not implemented between {} and {}", self, rhs),
        }
    }
}

impl Sub for Atom {
    type Output = Atom;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Float(_)) => todo!(),
            (Atom::Float(_), Atom::Str(_)) => todo!(),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a - b),
            (Atom::Int(a), Atom::Float(b)) => Atom::Float(a as f64 - b),
            (Atom::Float(a), Atom::Int(b)) => Atom::Float(a - b as f64),
            (Atom::Int(a), Atom::Int(b)) => Atom::Int(a - b),
            _ => todo!(),
        }
    }
}

impl Mul for Atom {
    type Output = Atom;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Float(_)) => todo!(),
            (Atom::Float(_), Atom::Str(_)) => todo!(),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a * b),
            (Atom::Int(a), Atom::Float(b)) => Atom::Float(a as f64 * b),
            (Atom::Float(a), Atom::Int(b)) => Atom::Float(a * b as f64),
            (Atom::Int(a), Atom::Int(b)) => Atom::Int(a * b),
            _ => todo!(),
        }
    }
}

impl Div for Atom {
    type Output = Atom;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Float(_)) => todo!(),
            (Atom::Float(_), Atom::Str(_)) => todo!(),
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a / b),
            (Atom::Int(a), Atom::Float(b)) => Atom::Float(a as f64 / b),
            (Atom::Float(a), Atom::Int(b)) => Atom::Float(a / b as f64),
            (Atom::Int(a), Atom::Int(b)) => {
                let res = a as f64 / b as f64;
                if res.fract() != 0.0 {
                    Atom::Float(res)
                } else {
                    Atom::Int(res as isize)
                }
            }
            _ => todo!(),
        }
    }
}

impl std::cmp::PartialOrd for Atom {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Float(_)) => todo!(),
            (Atom::Float(_), Atom::Str(_)) => todo!(),
            (Atom::Float(a), Atom::Float(b)) => a.partial_cmp(b),
            (Atom::Float(a), Atom::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Atom::Int(a), Atom::Float(b)) => (*a as f64).partial_cmp(b),
            (Atom::Int(a), Atom::Int(b)) => a.partial_cmp(b),
            _ => todo!(),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Str(s) => write!(f, "{}", s),
            Atom::Float(n) => write!(f, "{}", n),
            Atom::Int(n) => write!(f, "{}", n),
            Atom::Bool(b) => write!(f, "{}", b),
            Atom::Identifier(name) => write!(f, "(Identifier {})", name),
            Atom::Break => write!(f, "Break"),
            Atom::FnCall(FunctionCall { name, args }) => write!(f, "{}({:?})", name, args),
            Atom::Function(_) => write!(f, "FunctionData"),
            Atom::Array(a) => write!(f, "{:?}", a),
        }
    }
}

impl Atom {
    pub fn negate(self) -> Atom {
        match self {
            Atom::Str(_) => todo!(),
            Atom::Float(n) => Atom::Float(-1.0 * n),
            Atom::Int(n) => Atom::Int(-1 * n),
            Atom::Bool(b) => Atom::Bool(!b),
            _ => todo!(),
        }
    }

    pub fn modulus(&self, rhs: &Atom) -> Atom {
        match (self, rhs) {
            (Atom::Float(a), Atom::Float(b)) => Atom::Float(a % b),
            (Atom::Int(a), Atom::Int(b)) => Atom::Int(a % b),
            _ => todo!(),
        }
    }

    pub fn and(&self, rhs: &Atom) -> Atom {
        match (self, rhs) {
            (Atom::Bool(a), Atom::Bool(b)) => Atom::Bool(*a && *b),
            _ => todo!(),
        }
    }

    pub fn or(&self, rhs: &Atom) -> Atom {
        match (self, rhs) {
            (Atom::Bool(a), Atom::Bool(b)) => Atom::Bool(*a || *b),
            _ => todo!(),
        }
    }

    pub fn index(&self, rhs: &Atom) -> Atom {
        match (self, rhs) {
            (Atom::Array(a), Atom::Int(i)) if i >= &0 => {
                let i = *i as usize;
                if i < a.len() {
                    if let S::Atom(v) = &a[i] {
                        v.clone()
                    } else {
                        unreachable!();
                    }
                } else {
                    panic!(
                        "Tried getting {:?}th element of array with length {}",
                        i,
                        a.len()
                    )
                }
            }
            _ => panic!("Can't index {:?} by {:?}", self, rhs),
        }
    }

    pub fn access(&self, _rhs: &S) -> Atom {
        todo!();
        // match (self, rhs) {
        //     _ => panic!("Can't access field {} of {}", rhs, self),
        // }
    }

    pub fn compile(&self, scope: &CompileScope) {
        use Atom::*;

        match self {
            Int(i) => println!("Push {}", i),
            Identifier(n) => {
                let i = scope.vars.get(n).unwrap();
                println!("Get {}", i);
            }
            _ => todo!(),
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
