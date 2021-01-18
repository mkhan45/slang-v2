use crate::block::Block;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use crate::parser::S;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Str(String),
    Num(f64),
    Bool(bool),
    Identifier(String),
    FnCall(FunctionCall),
    Function(FunctionData),
    Break,
}

impl Add for Atom {
    type Output = Atom;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a + b),
            _ => todo!(),
        }
    }
}

impl Sub for Atom {
    type Output = Atom;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a - b),
            _ => todo!(),
        }
    }
}

impl Mul for Atom {
    type Output = Atom;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a * b),
            _ => todo!(),
        }
    }
}

impl Div for Atom {
    type Output = Atom;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a / b),
            _ => todo!(),
        }
    }
}

impl std::cmp::PartialOrd for Atom {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Atom::Str(_), Atom::Str(_)) => todo!(),
            (Atom::Str(_), Atom::Num(_)) => todo!(),
            (Atom::Num(_), Atom::Str(_)) => todo!(),
            (Atom::Num(a), Atom::Num(b)) => a.partial_cmp(b),
            _ => todo!(),
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Str(s) => write!(f, "{}", s),
            Atom::Num(n) => write!(f, "{}", n),
            Atom::Bool(b) => write!(f, "{}", b),
            Atom::Identifier(name) => write!(f, "(Identifier {})", name),
            Atom::Break => write!(f, "Break"),
            Atom::FnCall(FunctionCall { name, args }) => write!(f, "{}({:?})", name, args),
            Atom::Function(_) => write!(f, "FunctionData"),
        }
    }
}

impl Atom {
    pub fn negate(self) -> Atom {
        match self {
            Atom::Str(_) => todo!(),
            Atom::Num(n) => Atom::Num(-1.0 * n),
            _ => todo!(),
        }
    }

    pub fn modulus(&self, rhs: &Atom) -> Atom {
        match (self, rhs) {
            (Atom::Num(a), Atom::Num(b)) => Atom::Num(a % b),
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
