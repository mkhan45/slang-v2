use crate::{parse_expr, statement::Declaration, Lexer, Token, TokenType};

pub fn parse_declaration(lexer: &mut Lexer) -> Declaration {
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
            plus_or_minus: None,
        }
    } else {
        panic!("error on parsing assignment");
    }
}
