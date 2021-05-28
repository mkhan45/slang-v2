use crate::parser::*;
use crate::State;

pub mod atom;
use atom::Atom;

mod function;

type K<'a> = Box<dyn Fn(&'a mut State) -> Atom>;

pub fn eval_expr<'a>(expr: &'static S, state: &'static mut State) -> K<'a> {
    match expr {
        S::Atom(a) => match a {
            Atom::Identifier(name) => match state.get_variable(name) {
                Some(a) => Box::new(|_| a.clone()),
                None => panic!("Variable {} undefined in state {:?}", name, state),
            },
            Atom::FnCall(f) => Box::new(|_| function::eval_function_call(f, state).unwrap()),
            Atom::Array(arr) => {
                let new_arr = arr
                    .iter()
                    .map(|s| S::Atom(eval_expr(s, state)(state)))
                    .collect();
                Box::new(move |_| Atom::Array(new_arr).clone())
            }
            _ => Box::new(|_| a.clone()),
        },
        S::Cons(op, xs) => {
            let slice = xs.as_slice();
            match (op, slice) {
                (Op::Plus, [a, b, ..]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(move |_| a(state) + b(state))
                }
                (Op::Minus, [a, b, ..]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state) - b(state))
                }
                (Op::Minus, [a]) => {
                    let a = eval_expr(a, state);
                    Box::new(|_| a(state).negate())
                }
                (Op::Multiply, [a, b, ..]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state) * b(state))
                }
                (Op::Divide, [a, b, ..]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state) / b(state))
                }
                (Op::Negate, [a]) => {
                    let a = eval_expr(a, state);
                    Box::new(|_| a(state).negate())
                }
                (Op::Equal, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| Atom::Bool(a(state) == (b(state))))
                }
                (Op::NotEqual, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| Atom::Bool(a(state) != (b(state))))
                }
                (Op::Less, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| Atom::Bool(a(state) < (b(state))))
                }
                (Op::Greater, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| Atom::Bool(a(state) > (b(state))))
                }
                (Op::Mod, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state).modulus(&b(state)))
                }
                (Op::And, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state).and(&b(state)))
                }
                (Op::Or, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state).or(&b(state)))
                }
                (Op::Indexing, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    Box::new(|_| a(state).index(&b(state)))
                }
                (Op::Access, [a, b]) => {
                    let a = eval_expr(a, state);
                    let b = eval_expr(b, state);
                    // Box::new(|_| a(state).access(&b(state)))
                    todo!()
                }
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
            let mut top_state = State::default();
            $(
                let expr = parse_expr(&mut Lexer::new(scan_tokens($input)));
                assert_eq!(eval_expr(&expr, &mut top_state), $expected);
            )*
        }
    }

    #[test]
    fn test_eval() {
        eval_test!(
            "-2" => Atom::Int(-2),
            "5 + 4" => Atom::Int(5 + 4),
            "3 - 4 / 3" => Atom::Float(3.0 - 4.0 / 3.0),
            "3 + 5 * 4" => Atom::Int(3 + 5 * 4),
            "3 + 5 * 4 + -4 - -5" => Atom::Int(3 + 5 * 4 + -4 - -5),
            "3 * (4 + 5 * 8)" => Atom::Int(3 * (4 + 5 * 8)),
            "4.4 * (9 * 5 - 8 /     (3 - 4))" => Atom::Float(4.4 * (9.0 * 5.0 - 8.0 / (3.0 - 4.0))),
            "3.25/4 * 5" => Atom::Float(3.25 / 4.0 * 5.0),
            "(4.0 * 12.5) + 6.0 / (12.5 + 3.0)" => Atom::Float((4.0 * 12.5) + 6.0 / (12.5 + 3.0))
        );
    }
}
