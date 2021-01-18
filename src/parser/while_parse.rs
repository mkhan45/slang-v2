use crate::{parse_block, parse_expr, statement::While, Lexer, TokenType};

pub fn parse_while(lexer: &mut Lexer) -> While {
    lexer.next();
    assert_eq!(lexer.next().ty, TokenType::LParen);
    let cond = parse_expr(lexer);
    assert_eq!(lexer.next().ty, TokenType::RParen);
    assert_eq!(lexer.next().ty, TokenType::LBrace);
    let loop_block = parse_block(lexer);
    assert_eq!(lexer.next().ty, TokenType::RBrace);
    While { cond, loop_block }
}
