use crate::{parse_expr, statement::{Declaration, Stmt}, TokenType, Lexer};

pub fn parse_ident(lexer: &mut Lexer) -> Stmt {
    let nx = lexer.next();
    if lexer.peek().ty == TokenType::Assign {
        lexer.next();
        Stmt::Dec(Declaration {
            lhs: nx.lexeme,
            rhs: parse_expr(lexer),
            alias: false,
        })
    } else {
        lexer.prepend(nx);
        Stmt::ExprStmt(parse_expr(lexer))
    }
}
