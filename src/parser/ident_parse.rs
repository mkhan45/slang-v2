use crate::{parse_expr, statement::{Declaration, Stmt}, TokenType, Lexer};

pub fn parse_ident(lexer: &mut Lexer) -> Stmt {
    let nx = lexer.next();

    if [TokenType::Assign, TokenType::MinusAssign, TokenType::PlusAssign].contains(&lexer.peek().ty) {
        let plus_or_minus = match lexer.peek().ty {
            TokenType::Assign => None,
            TokenType::PlusAssign => Some(true),
            TokenType::MinusAssign => Some(false),
            _ => unreachable!(),
        };
        lexer.next();
        Stmt::Dec(Declaration {
            lhs: nx.lexeme,
            rhs: parse_expr(lexer),
            alias: false,
            plus_or_minus,
        })
    } else {
        lexer.prepend(nx);
        Stmt::ExprStmt(parse_expr(lexer))
    }
}
