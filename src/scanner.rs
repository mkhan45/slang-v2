pub mod token;
use token::*;

use crate::eval::atom::Atom;

use itertools::Itertools;

fn skip_comment(source: &[char]) -> Vec<char> {
    source
        .iter()
        .skip_while(|&&c| c != '\n' && c != '\r')
        .copied()
        .collect_vec()
}

fn num_token(source: &[char]) -> Vec<Token> {
    let cond = |&&c: &&char| c.is_numeric() || c == '.';
    let s: String = source.iter().take_while(cond).collect();

    let n = if let Ok(n) = s.parse::<isize>() {
        Token::new(TokenType::Literal(Atom::Int(n)), s, 0)
    } else {
        Token::new(TokenType::Unknown, s, 0)
    };

    let remaining = source.iter().skip_while(cond).copied().collect_vec();

    [vec![n], token_recurse(&remaining)].concat()
}

fn ident_token(source: &[char]) -> Vec<Token> {
    fn is_ident_char(c: &&char) -> bool {
        match &c {
            c if c.is_alphanumeric() => true,
            '_' => true,
            _ => false,
        }
    }

    let lex: String = source.iter().take_while(is_ident_char).collect();
    let remaining = source
        .iter()
        .skip_while(is_ident_char)
        .copied()
        .collect_vec();
    macro_rules! add_lexemes {
        ( $($lex:expr => $ty:expr),* ) => {
            match lex.as_str() {
                $( $lex => Token::from_ty($ty), )*
                _ => Token::new(TokenType::Identifier, lex.clone(), 0),
            };
        }
    }

    let token = add_lexemes!(
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "elif" => TokenType::Elif,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "fn" => TokenType::Function,
        "struct" => TokenType::Struct,
        "break" => TokenType::Break,
        "print" => TokenType::Print
    );

    [vec![token], token_recurse(&remaining)].concat()
}

// this could probably be many times faster but
// the evaluation will always take much longer
// than the scanner
fn token_recurse(source: &[char]) -> Vec<Token> {
    fn recur(ty: TokenType, s: &[char]) -> Vec<Token> {
        [vec![Token::from_ty(ty)], token_recurse(s)].concat()
    }

    match &source {
        [] | ['\n' | '\r'] => vec![],
        ['#', ..] => token_recurse(skip_comment(source).as_slice()),
        ['\n' | '\r', xs @ ..] => recur(TokenType::NewLine, xs),
        ['&', '&', xs @ ..] => recur(TokenType::And, xs),
        ['|', '|', xs @ ..] => recur(TokenType::Or, xs),
        ['+', '=', xs @ ..] => recur(TokenType::PlusAssign, xs),
        ['!', '=', xs @ ..] => recur(TokenType::BangEqual, xs),
        ['<', '=', xs @ ..] => recur(TokenType::LessEqual, xs),
        ['>', '=', xs @ ..] => recur(TokenType::GreaterEqual, xs),
        ['=', '=', xs @ ..] => recur(TokenType::Equal, xs),
        ['=', xs @ ..] => recur(TokenType::Assign, xs),
        ['<', xs @ ..] => recur(TokenType::Less, xs),
        ['>', xs @ ..] => recur(TokenType::Greater, xs),
        ['(', xs @ ..] => recur(TokenType::LParen, xs),
        [')', xs @ ..] => recur(TokenType::RParen, xs),
        ['{', xs @ ..] => recur(TokenType::LBrace, xs),
        ['}', xs @ ..] => recur(TokenType::RBrace, xs),
        [',', xs @ ..] => recur(TokenType::Comma, xs),
        ['*', xs @ ..] => recur(TokenType::Star, xs),
        ['/', xs @ ..] => recur(TokenType::Slash, xs),
        ['-', xs @ ..] => recur(TokenType::Minus, xs),
        ['+', xs @ ..] => recur(TokenType::Plus, xs),
        ['%', xs @ ..] => recur(TokenType::Percent, xs),
        ['.', xs @ ..] => recur(TokenType::Dot, xs),
        [';', xs @ ..] => recur(TokenType::Semicolon, xs),
        ['[', xs @ ..] => recur(TokenType::LBracket, xs),
        [']', xs @ ..] => recur(TokenType::RBracket, xs),
        ['!', xs @ ..] => recur(TokenType::Bang, xs),
        ls @ [c, ..] if c.is_numeric() => num_token(ls),
        ls @ [c, ..] if c.is_alphabetic() => ident_token(ls),
        [c, xs @ ..] if c.is_whitespace() => token_recurse(xs),
        c => panic!("Invalid input {:?}", c),
    }
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let slice = source.chars().collect_vec();
    let token_slice = token_recurse(&slice);
    token_slice.iter().cloned().collect_vec()
}

#[cfg(test)]
mod lexer_tests {
    use super::scan_tokens;
    use crate::eval::atom::Atom;
    use crate::scanner::token::*;

    macro_rules! test_lexer {
        ( $( $input:expr => $expected:expr ),* ) => {
            $(
                assert_eq!(scan_tokens($input), $expected.collect::<Vec<Token>>());
            )*
        }
    }

    fn string(s: &str) -> Token {
        Token::new(
            TokenType::Literal(Atom::Str(s.clone().to_string())),
            s.to_string(),
            0,
        )
    }

    fn identifier(n: &str) -> Token {
        Token::new(TokenType::Identifier, n.to_string(), 0)
    }

    #[test]
    fn test_lexer() {
        test_lexer!(
            "()" => [TokenType::LParen, TokenType::RParen].iter().map(|ty| Token::from_ty(ty.clone())),
            "(  ) \"asdf\" " => [Token::from_ty(TokenType::LParen),
            Token::from_ty(TokenType::RParen),
            string("asdf")].iter().cloned(),
            ">= \"hello#notcomment\" identifier_here # yes comment" =>
            [Token::from_ty(TokenType::GreaterEqual),
            string("hello#notcomment"),
            identifier("identifier_here")
            ].iter().cloned()
        );
    }
}
