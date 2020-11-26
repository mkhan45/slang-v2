use std::fmt;

use crate::scanner::token::*;

use crate::eval::atom::Atom;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

#[derive(Debug)]
pub enum S {
    Atom(Atom),
    Cons(Op, Vec<S>),
}

#[derive(Debug)]
pub enum Op {
    Plus,
    Minus,
    Negate,
    Multiply,
    Divide,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Negate => "!",
                Op::Plus => "+",
                Op::Minus => "-",
                Op::Multiply => "*",
                Op::Divide => "/",
            }
        )
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(mut tokens: Vec<Token>) -> Lexer {
        tokens.reverse();
        Lexer { tokens }
    }

    fn next(&mut self) -> Token {
        self.tokens
            .pop()
            .unwrap_or_else(|| Token::from_ty(TokenType::EOF))
    }

    fn peek(&mut self) -> Token {
        self.tokens
            .last()
            .cloned()
            .unwrap_or_else(|| Token::from_ty(TokenType::EOF))
    }
}

pub fn parse_expr(lexer: &mut Lexer) -> S {
    expr_bp(lexer, 0, 0)
}

fn is_prefix_op(t: &TokenType) -> bool {
    match t {
        TokenType::Minus | TokenType::Bang => true,
        _ => false,
    }
}

fn expr_bp(lexer: &mut Lexer, bp: u8, paren_depth: u16) -> S {
    let nx = lexer.next();
    let mut lhs = match nx.ty {
        TokenType::Literal(a) => S::Atom(a),
        t if is_prefix_op(&t) => {
            let op = match t {
                TokenType::Minus => Op::Minus,
                TokenType::Bang => Op::Negate,
                _ => unreachable!(),
            };
            let ((), r_bp) = prefix_binding_power(&op);
            let rhs = expr_bp(lexer, r_bp, paren_depth);
            S::Cons(op, vec![rhs])
        }
        TokenType::LParen => {
            let lhs = expr_bp(lexer, 0, paren_depth + 1);
            if lexer.next().ty == TokenType::RParen {
                lhs
            } else {
                panic!("Unbalanced left parenthesis")
            }
        }
        _ => panic!("Invalid token {}", nx),
    };

    loop {
        let nx = lexer.peek();
        let op = match nx.ty {
            TokenType::EOF | TokenType::WhiteSpace | TokenType::NewLine => break,
            TokenType::Plus => Op::Plus,
            TokenType::Minus => Op::Minus,
            TokenType::Slash => Op::Divide,
            TokenType::Star => Op::Multiply,
            TokenType::Bang => Op::Negate,
            TokenType::RParen => {
                if paren_depth < 1 {
                    panic!("Unbalanced right parenthesis");
                }
                break;
            }
            _ => unimplemented!(), // could be panic
        };

        let (l_bp, r_bp) = infix_binding_power(&op);
        if l_bp < bp {
            break;
        }

        lexer.next();
        let rhs = expr_bp(lexer, r_bp, paren_depth);

        lhs = S::Cons(op, vec![lhs, rhs]);
    }

    lhs
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Plus | Op::Minus => (1, 2),
        Op::Multiply | Op::Divide => (3, 4),
        _ => panic!("bad op {:?}", op),
    }
}

fn prefix_binding_power(op: &Op) -> ((), u8) {
    match op {
        Op::Minus => ((), 9),
        Op::Negate => ((), 10),
        _ => panic!("bad op: {:?}", op),
    }
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use crate::scan_tokens;
    use crate::Lexer;

    fn str_to_expr(s: &str) -> S {
        let tokens = scan_tokens(s);
        let mut lexer = Lexer::new(tokens);
        parse_expr(&mut lexer)
    }

    macro_rules! test_expr {
        ( $( $input:expr => $expected:expr ),* ) => {
            $(
                assert_eq!(str_to_expr($input).to_string(), $expected);
            )*
        }
    }

    #[test]
    fn test_parser() {
        test_expr!(
            "1" => "1",
            "5 + 5" => "(+ 5 5)",
            "1 + 2 * 3" => "(+ 1 (* 2 3))",
            "5 + 4 * 3 / 4 + 5" => "(+ (+ 5 (/ (* 4 3) 4)) 5)",
            "3 * (4 + 4)" => "(* 3 (+ 4 4))",
            "(5 + 5) * 4" => "(* (+ 5 5) 4)"
        );
    }
}
