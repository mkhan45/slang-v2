use crate::{parse_expr, statement::Declaration, Token, TokenType, Lexer};

pub fn parse_assignment(lexer: &mut Lexer) -> Declaration {
    if let Token {
        ty: TokenType::Identifier,
        lexeme: name,
        ..
    } = lexer.next()
    {
        assert_eq!(lexer.next().ty, TokenType::Assign);
        Declaration {
            lhs: name,
            rhs: parse_expr(lexer),
            alias: true,
            plus_or_minus: Some(false),
        }
    } else {
        panic!("error on parsing assignment");
    }
}
