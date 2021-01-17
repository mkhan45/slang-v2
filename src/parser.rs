use std::fmt;

use crate::{
    scanner::token::*,
    statement::Stmt,
};

use crate::eval::atom::Atom;

use crate::block::Block;

mod assignment_parse;
mod ident_parse;
mod if_parse;

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
    Equal,
    NotEqual,
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
                Op::Equal => "==",
                Op::NotEqual => "!=",
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

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn prepend(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

pub fn parse_block(lexer: &mut Lexer) -> Block {
    let add_stmt = || {
        if lexer.is_empty() || lexer.peek().ty == TokenType::RBrace {
            None
        } else {
            Some(parse_stmt(lexer))
        }
    };

    Block::new(std::iter::from_fn(add_stmt).collect())
}

pub fn parse_stmt(lexer: &mut Lexer) -> Stmt {
    match lexer.peek() {
        Token {
            ty: TokenType::NewLine,
            ..
        } => {
            lexer.next();
            parse_stmt(lexer)
        }
        Token {
            ty: TokenType::Print,
            ..
        } => {
            lexer.next();
            assert_eq!(lexer.next().ty, TokenType::LParen);
            let res = Stmt::PrintStmt(parse_expr(lexer));
            assert_eq!(lexer.next().ty, TokenType::RParen);
            assert_eq!(lexer.next().ty, TokenType::NewLine);
            res
        }
        Token {
            ty: TokenType::Let, ..
        } => {
            lexer.next();
            Stmt::Dec(assignment_parse::parse_assignment(lexer))
        }
        Token {
            ty: TokenType::Identifier,
            ..
        } => {
            ident_parse::parse_ident(lexer)
        }
        Token {
            ty: TokenType::If, ..
        } => {
            Stmt::IfStmt(if_parse::parse_if(lexer))
        }
        _t => Stmt::ExprStmt(parse_expr(lexer)),
    }
}

pub fn parse_expr(lexer: &mut Lexer) -> S {
    expr_bp(lexer, 0, 0)
}

fn is_prefix_op(t: &TokenType) -> bool {
    matches!(t, TokenType::Minus | TokenType::Bang)
}

fn expr_bp(lexer: &mut Lexer, bp: u8, paren_depth: u16) -> S {
    let nx = lexer.next();
    let mut lhs = match nx.ty {
        TokenType::Literal(a) => S::Atom(a),
        TokenType::Identifier => S::Atom(Atom::Identifier(nx.lexeme)),
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
            TokenType::EOF | TokenType::NewLine => {
                lexer.next();
                break;
            }
            TokenType::Plus => Op::Plus,
            TokenType::Minus => Op::Minus,
            TokenType::Slash => Op::Divide,
            TokenType::Star => Op::Multiply,
            TokenType::Bang => Op::Negate,
            TokenType::Equal => Op::Equal,
            TokenType::BangEqual => Op::NotEqual,
            TokenType::RParen => {
                // if paren_depth < 1 {
                //     panic!("Unbalanced right parenthesis");
                // }
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
        Op::Equal | Op::NotEqual => (0, 1),
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
