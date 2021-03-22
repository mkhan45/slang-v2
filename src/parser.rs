use crate::eval::atom::FunctionCall;
use crate::statement::CompileScope;
use crate::statement::Declaration;
use std::fmt;

use crate::{scanner::token::*, statement::Stmt};

use crate::eval::atom::Atom;

use crate::block::Block;

mod assignment_parse;
mod fn_parse;
mod for_parse;
mod ident_parse;
mod if_parse;
mod while_parse;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

#[derive(Debug, Clone)]
pub enum S {
    Atom(Atom),
    Cons(Op, Vec<S>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Plus,
    PlusAssign,
    Minus,
    MinusAssign,
    Negate,
    Multiply,
    Divide,
    Less,
    Equal,
    Greater,
    NotEqual,
    Mod,
    And,
    Or,
    Indexing,
    Access,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Op::Negate => "!",
                Op::Plus => "+",
                Op::PlusAssign => "+=",
                Op::Minus => "-",
                Op::MinusAssign => "-=",
                Op::Multiply => "*",
                Op::Divide => "/",
                Op::Less => "<",
                Op::Equal => "==",
                Op::Greater => ">",
                Op::NotEqual => "!=",
                Op::Mod => "%",
                Op::And => "&&",
                Op::Or => "||",
                Op::Indexing => "[]",
                Op::Access => "access",
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

impl S {
    pub fn compile(&self, compile_scope: &mut CompileScope) {
        use S::*;

        let mut c = |s: &S| s.compile(compile_scope);

        match self {
            Atom(a) => a.compile(&compile_scope),
            Cons(op, xs) => {
                let slice = xs.as_slice();
                match (op, slice) {
                    (Op::Plus, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("Add");
                    }
                    (Op::Minus, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("Sub");
                    }
                    (Op::Multiply, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("Mul");
                    }
                    (Op::Divide, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("Div");
                    }
                    (Op::Equal, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("EQ");
                    }
                    (Op::NotEqual, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("NE");
                    }
                    (Op::Less, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("LT");
                    }
                    (Op::Greater, [a, b, ..]) => {
                        c(a);
                        c(b);
                        println!("GT");
                    }
                    _ => {
                        dbg!(op);
                        todo!();
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
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
        if lexer.is_empty() {
            None
        } else {
            parse_stmt(lexer)
        }
    };

    Block::new(std::iter::from_fn(add_stmt).collect())
}

pub fn parse_stmt(lexer: &mut Lexer) -> Option<Stmt> {
    match lexer.peek() {
        Token {
            ty: TokenType::NewLine | TokenType::Semicolon,
            ..
        } => {
            lexer.next();
            parse_stmt(lexer)
        }
        Token {
            ty: TokenType::LBrace,
            ..
        } => {
            lexer.next();
            let block = Some(Stmt::Block(parse_block(lexer)));
            assert_eq!(lexer.next().ty, TokenType::RBrace);
            block
        }
        Token {
            ty: TokenType::RBrace,
            ..
        } => None,
        Token {
            ty: TokenType::Print,
            ..
        } => {
            lexer.next();
            assert_eq!(lexer.next().ty, TokenType::LParen);
            let res = Stmt::PrintStmt(parse_expr(lexer));
            assert_eq!(lexer.next().ty, TokenType::RParen);
            assert!({
                let next_ty = lexer.next().ty;
                next_ty == TokenType::NewLine || next_ty == TokenType::EOF
            });
            Some(res)
        }
        Token {
            ty: TokenType::Let, ..
        } => {
            lexer.next();
            Some(Stmt::Dec(assignment_parse::parse_declaration(lexer)))
        }
        Token {
            ty: TokenType::Identifier,
            ..
        } => Some(ident_parse::parse_ident(lexer)),
        Token {
            ty: TokenType::If, ..
        } => Some(Stmt::IfStmt(if_parse::parse_if(lexer))),
        Token {
            ty: TokenType::While,
            ..
        } => Some(Stmt::WhileStmt(while_parse::parse_while(lexer))),
        Token {
            ty: TokenType::For, ..
        } => Some(for_parse::parse_for(lexer)),
        Token {
            ty: TokenType::Break,
            ..
        } => {
            lexer.next();
            Some(Stmt::Break)
        }
        Token {
            ty: TokenType::Function,
            ..
        } => {
            let (fn_name, fn_data) = fn_parse::parse_fn_dec(lexer);
            Some(Stmt::Dec(Declaration {
                lhs: fn_name,
                rhs: S::Atom(Atom::Function(fn_data)),
                alias: true,
                plus_or_minus: None,
            }))
        }
        _t => Some(Stmt::ExprStmt(parse_expr(lexer))),
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
        TokenType::True => S::Atom(Atom::Bool(true)),
        TokenType::False => S::Atom(Atom::Bool(false)),
        TokenType::Identifier => match lexer.peek().ty {
            TokenType::LParen => {
                let args = fn_parse::parse_fn_call_args(lexer);
                S::Atom(Atom::FnCall(FunctionCall {
                    name: nx.lexeme,
                    args,
                }))
            }
            _ => S::Atom(Atom::Identifier(nx.lexeme)),
        },
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
        TokenType::LBracket => {
            let next_elem = || {
                if lexer.peek().ty != TokenType::RBracket {
                    let res = parse_expr(lexer);
                    if lexer.peek().ty == TokenType::Comma {
                        lexer.next();
                    }
                    Some(res)
                } else {
                    assert_eq!(lexer.next().ty, TokenType::RBracket);
                    None
                }
            };

            let arr_elements: Vec<S> = std::iter::from_fn(next_elem).collect();
            S::Atom(Atom::Array(arr_elements))
        }
        _ => panic!("Invalid token {}", nx),
    };

    loop {
        let nx = lexer.peek();
        let op = match nx.ty {
            TokenType::EOF
            | TokenType::NewLine
            | TokenType::Semicolon
            | TokenType::Comma
            | TokenType::RBracket => {
                break;
            }
            TokenType::Plus => Op::Plus,
            TokenType::PlusAssign => Op::PlusAssign,
            TokenType::Minus => Op::Minus,
            TokenType::MinusAssign => Op::MinusAssign,
            TokenType::Slash => Op::Divide,
            TokenType::Star => Op::Multiply,
            TokenType::Bang => Op::Negate,
            TokenType::Equal => Op::Equal,
            TokenType::Less => Op::Less,
            TokenType::Greater => Op::Greater,
            TokenType::BangEqual => Op::NotEqual,
            TokenType::Percent => Op::Mod,
            TokenType::And => Op::And,
            TokenType::Or => Op::Or,
            TokenType::LBracket => Op::Indexing,
            TokenType::Dot => Op::Access,
            TokenType::RParen | TokenType::RBrace => {
                // if paren_depth < 1 {
                //     panic!("Unbalanced right parenthesis");
                // }
                break;
            }
            t => unimplemented!("Operator: {:?}, lhs: {:?}", t, lhs), // could be panic
        };

        if let Some((l_bp, ())) = postfix_binding_power(&op) {
            if l_bp < bp {
                break;
            }
            lexer.next();

            lhs = if op == Op::Indexing {
                let rhs = parse_expr(lexer);
                assert_eq!(lexer.next().ty, TokenType::RBracket);
                S::Cons(op, vec![lhs, rhs])
            } else {
                S::Cons(op, vec![lhs])
            };

            continue;
        }

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

fn postfix_binding_power(op: &Op) -> Option<(u8, ())> {
    let res = match op {
        Op::Indexing => (9, ()),
        _ => return None,
    };
    Some(res)
}

fn infix_binding_power(op: &Op) -> (u8, u8) {
    match op {
        Op::Access => (8, 9),
        Op::Plus | Op::Minus => (4, 5),
        Op::Multiply | Op::Divide => (6, 7),
        Op::Mod => (2, 3),
        Op::And | Op::Or => (1, 2),
        Op::Equal | Op::NotEqual | Op::Less | Op::Greater => (0, 1),
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
