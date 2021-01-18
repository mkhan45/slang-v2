use crate::statement::Stmt;
use crate::Atom;
use crate::{block::Block, parse_stmt};
use crate::{parse_block, parse_expr, statement::While, Lexer, TokenType};

use super::S;

pub fn parse_for(lexer: &mut Lexer) -> Stmt {
    lexer.next();

    assert_eq!(lexer.next().ty, TokenType::LParen);
    let init_statement = if lexer.peek().ty != TokenType::Semicolon {
        let stmt = Some(parse_stmt(lexer));
        stmt
    } else {
        None
    };
    assert_eq!(lexer.next().ty, TokenType::Semicolon);

    let cond = if lexer.peek().ty != TokenType::Semicolon {
        let expr = Some(parse_expr(lexer));
        expr
    } else {
        None
    };
    assert_eq!(lexer.next().ty, TokenType::Semicolon);

    let incr = if lexer.peek().ty != TokenType::RParen {
        Some(parse_stmt(lexer))
    } else {
        None
    };
    assert_eq!(lexer.next().ty, TokenType::RParen);

    assert_eq!(lexer.next().ty, TokenType::LBrace);
    let mut loop_block = parse_block(lexer);
    assert_eq!(lexer.next().ty, TokenType::RBrace);

    if let Some(Some(s)) = incr {
        loop_block.statements.push(s);
    }

    let while_stmt = While {
        cond: cond.unwrap_or(S::Atom(Atom::Bool(true))),
        loop_block,
    };

    if let Some(Some(s)) = init_statement {
        Stmt::Block(Block::new(vec![s, Stmt::WhileStmt(while_stmt)]))
    } else {
        Stmt::Block(Block::new(vec![Stmt::WhileStmt(while_stmt)]))
    }
}
