use crate::{block::Block, parse_block, parse_expr, statement::If, Lexer, TokenType};

pub fn parse_if(lexer: &mut Lexer) -> If {
    lexer.next();
    assert_eq!(lexer.next().ty, TokenType::LParen);
    let cond = parse_expr(lexer);
    assert_eq!(lexer.next().ty, TokenType::RParen);
    assert_eq!(lexer.next().ty, TokenType::LBrace);
    let then_block = parse_block(lexer);
    assert_eq!(lexer.next().ty, TokenType::RBrace);
    if lexer.peek().ty == TokenType::Else {
        lexer.next();
        assert_eq!(lexer.next().ty, TokenType::LBrace);
        let else_block = parse_block(lexer);
        assert_eq!(lexer.next().ty, TokenType::RBrace);
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
