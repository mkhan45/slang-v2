pub mod token;
use token::*;

use crate::eval::atom::Atom;

use itertools::Itertools;

fn skip_comment(source: &[char]) -> Vec<char> {
    source.iter().skip_while(|&&c| c != '\n' && c != '\r').copied().collect_vec()
}

fn string_token(source: &[char]) -> Vec<Token> {
    let cond = |&&c: &&char| c != '\"';
    let s: String = source.iter().take_while(cond).collect();
    let remaining = source.iter().skip_while(cond).skip(1).copied().collect_vec();
    [vec![Token::new(TokenType::Literal(Atom::Str(s.clone())), s, 0)], token_recurse(&remaining)].concat()
}

fn num_token(source: &[char]) -> Vec<Token> {
    let cond = |&&c: &&char| c.is_numeric() || c == '.';
    let s: String = source.iter().take_while(cond).collect();

    let n = if let Ok(n) = s.parse::<f32>() {
        Token::new(TokenType::Literal(Atom::Num(n)), s, 0)
    } else if let Ok(n) = s.parse::<isize>() {
        Token::new(TokenType::Literal(Atom::Num(n as f32)), s, 0)
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
    let remaining = source.iter().skip_while(is_ident_char).copied().collect_vec();
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
        "print" => TokenType::Print
    );

    [vec![token], token_recurse(&remaining)].concat()
}

fn token_recurse(source: &[char]) -> Vec<Token> {
    match &source {
        [] | ['\n' | '\r'] => vec![],
        ['#', ..] => token_recurse(skip_comment(source).as_slice()),
        ['\n' | '\r', xs@..] => [vec![Token::from_ty(TokenType::NewLine)], token_recurse(xs)].concat(),
        ['+', '=', xs@..] => [vec![Token::from_ty(TokenType::PlusAssign)], token_recurse(xs)].concat(),
        ['!', '=', xs@..] => [vec![Token::from_ty(TokenType::BangEqual)], token_recurse(xs)].concat(),
        ['<', '=', xs@..] => [vec![Token::from_ty(TokenType::LessEqual)], token_recurse(xs)].concat(),
        ['>', '=', xs@..] => [vec![Token::from_ty(TokenType::GreaterEqual)], token_recurse(xs)].concat(),
        ['=', '=', xs@..] => [vec![Token::from_ty(TokenType::Equal)], token_recurse(xs)].concat(),
        ['=', xs@..] => [vec![Token::from_ty(TokenType::Assign)], token_recurse(xs)].concat(),
        ['(', xs@..] => [vec![Token::from_ty(TokenType::LParen)], token_recurse(xs)].concat(),
        [')', xs@..] => [vec![Token::from_ty(TokenType::RParen)], token_recurse(xs)].concat(),
        ['{', xs@..] => [vec![Token::from_ty(TokenType::LBrace)], token_recurse(xs)].concat(),
        ['}', xs@..] => [vec![Token::from_ty(TokenType::RBrace)], token_recurse(xs)].concat(),
        [',', xs@..] => [vec![Token::from_ty(TokenType::Comma)], token_recurse(xs)].concat(),
        ['*', xs@..] => [vec![Token::from_ty(TokenType::Star)], token_recurse(xs)].concat(),
        ['/', xs@..] => [vec![Token::from_ty(TokenType::Slash)], token_recurse(xs)].concat(),
        ['-', xs@..] => [vec![Token::from_ty(TokenType::Minus)], token_recurse(xs)].concat(),
        ['+', xs@..] => [vec![Token::from_ty(TokenType::Plus)], token_recurse(xs)].concat(),
        ['.', xs@..] => [vec![Token::from_ty(TokenType::Dot)], token_recurse(xs)].concat(),
        ['\"', xs@..] => string_token(xs),
        ls@[c, ..] if c.is_numeric() => num_token(ls),
        ls@[c, ..] if c.is_alphabetic() => ident_token(ls),
        [c, xs@..] if c.is_whitespace() => token_recurse(xs),
        c@_ => panic!("Invalid input {:?}", c),
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
