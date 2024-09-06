use std::{iter::Peekable, str::Chars};

use crate::scanner::{
    comment, identifier, keywords, number, string,
    token::{Error, Token, Type},
};

fn next_char_match(char: char, chars: &mut Peekable<Chars>) -> bool {
    if chars.next_if_eq(&char).is_some() {
        return true;
    }

    false
}

#[derive(Debug)]
pub struct ScanTokens {
    pub tokens: Vec<Token>,
    pub errors: Vec<Error>,
}

pub fn scan_tokens(content: &str) -> ScanTokens {
    let mut chars = content.chars().peekable();

    let mut tokens: Vec<Token> = vec![];

    let mut errors: Vec<Error> = vec![];

    let mut line = 1;

    while let Some(char) = chars.next() {
        match char {
            ')' => tokens.push(Token::new(Type::RightParen, ")", None)),
            '(' => tokens.push(Token::new(Type::LeftParen, "(", None)),
            '}' => tokens.push(Token::new(Type::RightBrace, "}", None)),
            '{' => tokens.push(Token::new(Type::LeftBrace, "{", None)),
            '*' => tokens.push(Token::new(Type::Star, "*", None)),
            '.' => tokens.push(Token::new(Type::Dot, ".", None)),
            ',' => tokens.push(Token::new(Type::Comma, ",", None)),
            '+' => tokens.push(Token::new(Type::Plus, "+", None)),
            '-' => tokens.push(Token::new(Type::Minus, "-", None)),
            ';' => tokens.push(Token::new(Type::Semicolon, ";", None)),
            '/' => {
                if next_char_match('/', &mut chars) {
                    comment::scan(&mut chars);
                } else {
                    tokens.push(Token::new(Type::Slash, "/", None));
                }
            }
            '!' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::BangEqual, "!=", None));
                } else {
                    tokens.push(Token::new(Type::Bang, "!", None));
                }
            }
            '=' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::EqualEqual, "==", None));
                } else {
                    tokens.push(Token::new(Type::Equal, "=", None));
                }
            }
            '<' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::LessEqual, "<=", None));
                } else {
                    tokens.push(Token::new(Type::Less, "<", None));
                }
            }
            '>' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::GreaterEqual, ">=", None));
                } else {
                    tokens.push(Token::new(Type::Greater, ">", None));
                }
            }
            '"' => match string::scan(&mut chars, &mut line) {
                Ok((text, value)) => {
                    tokens.push(Token::new(Type::String, text.as_str(), Some(value)));
                }
                Err(error) => errors.push(error),
            },
            '0'..='9' => {
                let (text, value) = number::scan(char, &mut chars);
                tokens.push(Token::new(Type::Number, text.as_str(), Some(value)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let text = identifier::scan(char, &mut chars);
                let keyword = keywords::map().get(text.as_str());

                if let Some(token_type) = keyword {
                    tokens.push(Token::new(token_type.clone(), text.as_str(), None));
                } else {
                    tokens.push(Token::new(Type::Identifier, text.as_str(), None));
                }
            }
            ' ' | '\t' => {}
            '\n' => {
                line += 1;
            }
            _ => {
                errors.push(Error {
                    message: format!("Unexpected character: {char}"),
                    line,
                });
            }
        }
    }

    ScanTokens { tokens, errors }
}
