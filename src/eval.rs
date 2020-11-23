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

#[cfg(test)]
mod eval_tests {
    use super::*;
    use crate::parser::parse_expr;
    use crate::scan_tokens;

    macro_rules! eval_test {
        ( $( $input:expr => $expected:expr ),* ) => {
            $(
                let expr = parse_expr(&mut Lexer::new(scan_tokens($input)));
                assert_eq!(eval_expr(&expr), $expected);
            )*
        }
    }

    #[test]
    fn test_eval() {
        eval_test!(
            "-2" => Atom::Num(-2.0),
            "5 + 5" => Atom::Num(5.0 + 5.0),
            "3 - 4 / 3" => Atom::Num(3.0 - 4.0 / 3.0),
            "3 + 5 * 4" => Atom::Num(3.0 + 5.0 * 4.0),
            "3 + 5 * 4 + -4 - -5" => Atom::Num(3.0 + 5.0 * 4.0 + -4.0 - -5.0),
            "3 * (4 + 5 * 8)" => Atom::Num(3.0 * (4.0 + 5.0 * 8.0)),
            "4.4 * (9 * 5 - 8 / (3 - 4))" => Atom::Num(4.4 * (9.0 * 5.0 - 8.0 / (3.0 - 4.0)))
        );
    }
}
