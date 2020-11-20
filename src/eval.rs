use crate::parser::*;

pub mod atom;
use atom::Atom;

pub fn eval_expr(expr: &S) -> Atom {
    match expr {
        S::Atom(a) => a.clone(),
        S::Cons(Op::Plus, xs) => eval_expr(&xs[0]) + eval_expr(&xs[1]),
        S::Cons(Op::Minus, xs) => eval_expr(&xs[0]) - eval_expr(&xs[1]),
        S::Cons(Op::Multiply, xs) => eval_expr(&xs[0]) * eval_expr(&xs[1]),
        S::Cons(Op::Divide, xs) => eval_expr(&xs[0]) / eval_expr(&xs[1]),
    }
}
