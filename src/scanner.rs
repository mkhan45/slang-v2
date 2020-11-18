pub mod token;
use token::*;

use itertools::Itertools;

type Scanner<'a> = std::iter::Peekable<std::str::Chars<'a>>;

fn scan_string(source: &mut Scanner, line: usize) -> Token {
    let res = source
        .peeking_take_while(|&c| c != '\"')
        .collect::<String>();
    if source.next().is_some() {
        let token = Token::new(TokenType::Str, res.clone(), Box::new(res), line);
        token
    } else {
        Token::new(TokenType::Unknown, res, Box::new("".to_string()), line)
    }
}

// TODO fix parens
fn scan_number(first: char, source: &mut Scanner, line: usize) -> Token {
    let lexeme = std::iter::once(first)
        .chain(source.peeking_take_while(|&c| c.is_numeric() || c == '.'))
        .collect::<String>();

    if let Ok(n) = lexeme.parse::<f32>() {
        Token::new(TokenType::Number, lexeme.clone(), Box::new(n), line)
    } else if let Ok(n) = lexeme.parse::<isize>() {
        Token::new(TokenType::Number, lexeme.clone(), Box::new(n as f32), line)
    } else {
        Token::new(TokenType::Unknown, lexeme.clone(), Box::new(lexeme), line)
    }
}

fn scan_identifier(first: char, source: &mut Scanner, line: usize) -> Token {
    let lexeme = std::iter::once(first)
        .chain(source.take_while(|&c| !c.is_whitespace()))
        .collect::<String>();
    macro_rules! add_lexemes {
        ( $($lex:expr => $ty:expr),* ) => {
            match lexeme.as_str() {
                $( $lex => Token::from_ty($ty), )*
                _ => Token::new(TokenType::Identifier, lexeme.clone(), Box::new(lexeme), line),
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
    while let Some(c) = source.next() {
        if c == '\n' {
            break;
        }
    }
}

pub fn scan_tokens(source: &String) -> Vec<Token> {
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
            (Some(c), _n_opt) => {
                if c.is_numeric() {
                    Some(scan_number(c, &mut char_iter, line))
                } else if c.is_alphabetic() {
                    Some(scan_identifier(c, &mut char_iter, line))
                } else if c.is_whitespace() {
                    Some(Token::from_ty(TokenType::WhiteSpace))
                } else {
                    Some(Token::new(
                        TokenType::Unknown,
                        c.to_string(),
                        Box::new(c.to_string()),
                        line,
                    ))
                }
            }
            (_, _) => None,
        }
    };

    let token_iter = std::iter::from_fn(next_token);

    token_iter
        .filter(|t| ![TokenType::WhiteSpace, TokenType::Hash].contains(&t.ty))
        .collect::<Vec<Token>>()
}
