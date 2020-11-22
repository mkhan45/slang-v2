pub mod token;
use token::*;

use crate::eval::atom::Atom;

use itertools::Itertools;

type Scanner<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn scan_string(source: &mut Scanner, line: usize) -> Token {
    let res = source
        .peeking_take_while(|&c| c != '\"')
        .collect::<String>();
    if source.next().is_some() {
        Token::new(TokenType::Literal(Atom::Str(res.clone())), res, line)
    } else {
        Token::unknown(line)
    }
}

// TODO fix parens
fn scan_number(first: char, source: &mut Scanner, line: usize) -> Token {
    let lexeme = std::iter::once(first)
        .chain(source.peeking_take_while(|&c| c.is_numeric() || c == '.'))
        .collect::<String>();

    if let Ok(n) = lexeme.parse::<f32>() {
        Token::new(TokenType::Literal(Atom::Num(n)), lexeme.clone(), line)
    } else if let Ok(n) = lexeme.parse::<isize>() {
        Token::new(
            TokenType::Literal(Atom::Num(n as f32)),
            lexeme.clone(),
            line,
        )
    } else {
        Token::new(TokenType::Unknown, lexeme.clone(), line)
    }
}

fn scan_identifier(first: char, source: &mut Scanner, line: usize) -> Token {
    fn is_ident_char(c: &char) -> bool {
        match c {
            c if c.is_alphanumeric() => true,
            '_' => true,
            _ => false,
        }
    }

    let lexeme = std::iter::once(first)
        .chain(source.peeking_take_while(is_ident_char))
        .collect::<String>();
    macro_rules! add_lexemes {
        ( $($lex:expr => $ty:expr),* ) => {
            match lexeme.as_str() {
                $( $lex => Token::from_ty($ty), )*
                _ => Token::new(TokenType::Identifier, lexeme.clone(), line),
            }
        }
    }

    add_lexemes!(
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "elif" => TokenType::Elif,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "fn" => TokenType::Function,
        "struct" => TokenType::Struct
    )
}

fn skip_comment(source: &mut Scanner) {
    source.peeking_take_while(|&c| c != '\n').for_each(drop);
}

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut char_iter = (source).chars().peekable();
    let mut line = 1;

    let next_token = || {
        let c = char_iter.next();
        let peek = char_iter.peek();
        match (c, peek) {
            (Some('\n' | '\r'), _) => {
                line += 1;
                Some(Token::from_ty(TokenType::NewLine))
            }
            (Some('('), _) => Some(Token::from_ty(TokenType::LParen)),
            (Some(')'), _) => Some(Token::from_ty(TokenType::RParen)),
            (Some('{'), _) => Some(Token::from_ty(TokenType::LBrace)),
            (Some('}'), _) => Some(Token::from_ty(TokenType::RBrace)),
            (Some(','), _) => Some(Token::from_ty(TokenType::Comma)),
            (Some('*'), _) => Some(Token::from_ty(TokenType::Star)),
            (Some('/'), _) => Some(Token::from_ty(TokenType::Slash)),
            (Some('.'), _) => Some(Token::from_ty(TokenType::Dot)),
            (Some('#'), _) => {
                skip_comment(&mut char_iter);
                Some(Token::from_ty(TokenType::Hash))
            }
            (Some('+'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::PlusAssign))
            }
            (Some('+'), _) => Some(Token::from_ty(TokenType::Plus)),
            (Some('-'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::MinusAssign))
            }
            (Some('-'), _) => Some(Token::from_ty(TokenType::Minus)),
            (Some('!'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::BangEqual))
            }
            (Some('!'), _) => Some(Token::from_ty(TokenType::Bang)),
            (Some('<'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::LessEqual))
            }
            (Some('<'), _) => Some(Token::from_ty(TokenType::Less)),
            (Some('>'), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::GreaterEqual))
            }
            (Some('>'), _) => Some(Token::from_ty(TokenType::Greater)),
            (Some('='), Some('=')) => {
                char_iter.next();
                Some(Token::from_ty(TokenType::Equal))
            }
            (Some('='), _) => Some(Token::from_ty(TokenType::Assign)),
            (Some('\"'), _) => Some(scan_string(&mut char_iter, line)),
            (Some(c), _) if c.is_numeric() => Some(scan_number(c, &mut char_iter, line)),
            (Some(c), _) if c.is_alphabetic() => Some(scan_identifier(c, &mut char_iter, line)),
            (Some(c), _) if c.is_whitespace() => Some(Token::from_ty(TokenType::WhiteSpace)),
            (Some(_), _) => Some(Token::unknown(line)),
            (_, _) => None,
        }
    };

    let token_iter = std::iter::from_fn(next_token);

    token_iter
        .filter(|t| ![TokenType::WhiteSpace, TokenType::Hash].contains(&t.ty))
        .collect::<Vec<Token>>()
}

#[allow(unused_imports)]
mod lexer_tests {
    use super::scan_tokens;
    use crate::eval::atom::Atom;
    use crate::scanner::token::*;

    // for some reason it thinks everything only used by tests is dead code
    // so none of the code here is actually dead

    #[allow(unused_macros)]
    macro_rules! test_lexer {
        ( $( $input:expr => $expected:expr ),* ) => {
            $(
                assert_eq!(scan_tokens($input), $expected.collect::<Vec<Token>>());
            )*
        }
    }

    #[allow(dead_code)]
    fn string(s: &str) -> Token {
        Token::new(
            TokenType::Literal(Atom::Str(s.clone().to_string())),
            s.to_string(),
            1,
        )
    }

    #[allow(dead_code)]
    fn identifier(n: &str) -> Token {
        Token::new(TokenType::Identifier, n.to_string(), 1)
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
