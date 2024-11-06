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
            ')' => tokens.push(Token::new(Type::RightParen, ")", None, line)),
            '(' => tokens.push(Token::new(Type::LeftParen, "(", None, line)),
            '}' => tokens.push(Token::new(Type::RightBrace, "}", None, line)),
            '{' => tokens.push(Token::new(Type::LeftBrace, "{", None, line)),
            '*' => tokens.push(Token::new(Type::Star, "*", None, line)),
            '.' => tokens.push(Token::new(Type::Dot, ".", None, line)),
            ',' => tokens.push(Token::new(Type::Comma, ",", None, line)),
            '+' => tokens.push(Token::new(Type::Plus, "+", None, line)),
            '-' => tokens.push(Token::new(Type::Minus, "-", None, line)),
            ';' => tokens.push(Token::new(Type::Semicolon, ";", None, line)),
            '/' => {
                if next_char_match('/', &mut chars) {
                    comment::scan(&mut chars);
                } else {
                    tokens.push(Token::new(Type::Slash, "/", None, line));
                }
            }
            '!' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::BangEqual, "!=", None, line));
                } else {
                    tokens.push(Token::new(Type::Bang, "!", None, line));
                }
            }
            '=' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::EqualEqual, "==", None, line));
                } else {
                    tokens.push(Token::new(Type::Equal, "=", None, line));
                }
            }
            '<' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::LessEqual, "<=", None, line));
                } else {
                    tokens.push(Token::new(Type::Less, "<", None, line));
                }
            }
            '>' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(Token::new(Type::GreaterEqual, ">=", None, line));
                } else {
                    tokens.push(Token::new(Type::Greater, ">", None, line));
                }
            }
            '"' => match string::scan(&mut chars, &mut line) {
                Ok((text, value)) => {
                    tokens.push(Token::new(Type::String, text.as_str(), Some(value), line));
                }
                Err(error) => errors.push(error),
            },
            '0'..='9' => {
                let (text, value) = number::scan(char, &mut chars);
                tokens.push(Token::new(Type::Number, text.as_str(), Some(value), line));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let text = identifier::scan(char, &mut chars);
                let keyword = keywords::map().get(text.as_str());

                if let Some(token_type) = keyword {
                    tokens.push(Token::new(*token_type, text.as_str(), None, line));
                } else {
                    tokens.push(Token::new(Type::Identifier, text.as_str(), None, line));
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
