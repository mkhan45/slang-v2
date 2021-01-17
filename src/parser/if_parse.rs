use crate::{parse_block, parse_expr, statement::If, TokenType, Lexer, block::Block};

pub fn parse_if(lexer: &mut Lexer) -> If {
    lexer.next();
    assert_eq!(lexer.next().ty, TokenType::LParen);
    let cond = parse_expr(lexer);
    dbg!();
    assert_eq!(lexer.next().ty, TokenType::RParen);
    assert_eq!(lexer.next().ty, TokenType::LBrace);
    dbg!();
    let then_block = parse_block(lexer);
    assert_eq!(lexer.next().ty, TokenType::RBrace);
    dbg!();
    if lexer.peek().ty == TokenType::Else {
        lexer.next();
        dbg!();
        assert_eq!(lexer.next().ty, TokenType::LBrace);
        let else_block = parse_block(lexer);
        assert_eq!(lexer.next().ty, TokenType::RBrace);
        dbg!();
        If {
            cond,
            then_block,
            else_block,
        }
    } else {
        let else_block = Block::new(Vec::with_capacity(0));
        If {
            cond,
            then_block,
            else_block,
        }
    }
}
