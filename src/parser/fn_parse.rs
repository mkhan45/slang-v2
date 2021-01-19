use crate::eval::atom::FunctionData;
use crate::Token;
use crate::S;
use crate::{parse_expr, Lexer, TokenType};

use super::parse_block;

pub fn parse_fn_dec(lexer: &mut Lexer) -> (String, FunctionData) {
    assert_eq!(lexer.next().ty, TokenType::Function);

    let fn_name = if let Token {
        ty: TokenType::Identifier,
        lexeme: name,
        ..
    } = lexer.next()
    {
        name
    } else {
        panic!("Invalid fn declaration")
    };

    let arg_names = parse_fn_dec_args(lexer);

    assert_eq!(lexer.next().ty, TokenType::LBrace);
    let fn_block = parse_block(lexer);
    assert_eq!(lexer.next().ty, TokenType::RBrace);

    (
        fn_name,
        FunctionData {
            arg_names,
            fn_block,
        },
    )
}

pub fn parse_fn_dec_args(lexer: &mut Lexer) -> Vec<String> {
    assert_eq!(lexer.next().ty, TokenType::LParen);
    let args: Vec<String> = std::iter::from_fn(|| match lexer.peek().ty {
        TokenType::RParen => None,
        TokenType::Identifier => {
            let arg_name = lexer.next().lexeme;
            if lexer.peek().ty == TokenType::Comma {
                lexer.next();
            }
            Some(arg_name)
        }
        _ => panic!("Invalid fn args in declaration"),
    })
    .collect();
    assert_eq!(lexer.next().ty, TokenType::RParen);
    args
}

pub fn parse_fn_call_args(lexer: &mut Lexer) -> Vec<S> {
    assert_eq!(lexer.next().ty, TokenType::LParen);
    let args: Vec<S> = std::iter::from_fn(|| match lexer.peek().ty {
        TokenType::RParen => None,
        _ => {
            let expr = Some(parse_expr(lexer));
            if lexer.peek().ty == TokenType::Comma {
                lexer.next();
            }
            expr
        }
    })
    .collect();
    assert_eq!(lexer.next().ty, TokenType::RParen);
    args
}
