use crate::parser::*;

pub mod atom;
use atom::Atom;

pub fn eval_expr(expr: &S) -> Atom {
    match expr {
        S::Atom(a) => a.clone(),
        S::Cons(op, xs) => {
            let slice = xs.as_slice();
            match (op, slice) {
                (Op::Plus, [a, b, ..]) => eval_expr(&a) + eval_expr(&b),
                (Op::Minus, [a, b, ..]) => eval_expr(&a) - eval_expr(&b),
                (Op::Minus, [a]) => eval_expr(&a).negate(),
                (Op::Multiply, [a, b, ..]) => eval_expr(&a) * eval_expr(&b),
                (Op::Divide, [a, b, ..]) => eval_expr(&a) / eval_expr(&b),
                _ => panic!("invalid expr: {}", expr),
            }
        }
    }
}
