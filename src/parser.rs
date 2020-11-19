use std::{rc::Rc, fmt};

use crate::scanner::token::*;

use itertools::Itertools;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

// pub enum S<'a> {
//     Atom(&'a Token),
//     Cons(&'a Token, Vec<S<'a>>),
// }

// impl<'a> fmt::Display for S<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             S::Atom(i) => write!(f, "{}", i),
//             S::Cons(head, rest) => {
//                 write!(f, "({}", head)?;
//                 for s in rest {
//                     write!(f, " {}", s)?
//                 }
//                 write!(f, ")")
//             }
//         }
//     }
// }

// pub fn expr<'a>(tokens: &'a mut impl Iterator<Item=&'a Token>) -> S {
//     expr_bp(tokens, 0)
// }

// fn expr_bp<'a>(tokens: &'a mut impl Iterator<Item=&'a Token>, bp: u8) -> S {
//     let mut tokens = tokens.peekable();

//     let lhs = match tokens.next() {
//         Some(t@Token {ty: TokenType::Number, ..}) => S::Atom(t),
//         _ => unimplemented!(),
//     };

//     while let (Some(t), p) = (tokens.next(), tokens.peek()) {
//         let op = match p {
//             Some(Token{ty: TokenType::Plus, ..}) => TokenType::Plus,
//             None => break,
//             Some(t) => panic!("bad token: {}", t)
//         };

//         let (l_bp, r_bp) = infix_binding_power(op);
//         if l_bp < bp {
//             break;
//         }

//         tokens.next();
//         let rhs = expr_bp(&mut tokens, r_bp);
//     }

//     lhs
// }

// fn infix_binding_power(op: TokenType) -> (u8, u8) {
//     match op {
//         TokenType::Plus => (1, 2),
//         _ => panic!("bad op: {:?}", op)
//     }
// }
