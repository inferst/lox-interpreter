use std::{iter::Peekable, str::Chars};

use crate::scanner::{comment, identifier, keywords, number, string, token};

fn next_char_match(char: char, chars: &mut Peekable<Chars>) -> bool {
    if chars.next_if_eq(&char).is_some() {
        return true;
    }

    false
}

pub fn scan_tokens(content: &str) {
    let mut chars = content.chars().peekable();

    let mut tokens: Vec<token::Token> = vec![];

    let mut errors: Vec<token::Error> = vec![];

    let mut line = 1;

    while let Some(char) = chars.next() {
        match char {
            ')' => tokens.push(token::new(token::Type::RightParen, ")", None)),
            '(' => tokens.push(token::new(token::Type::LeftParen, "(", None)),
            '}' => tokens.push(token::new(token::Type::RightBrace, "}", None)),
            '{' => tokens.push(token::new(token::Type::LeftBrace, "{", None)),
            '*' => tokens.push(token::new(token::Type::Star, "*", None)),
            '.' => tokens.push(token::new(token::Type::Dot, ".", None)),
            ',' => tokens.push(token::new(token::Type::Comma, ",", None)),
            '+' => tokens.push(token::new(token::Type::Plus, "+", None)),
            '-' => tokens.push(token::new(token::Type::Minus, "-", None)),
            ';' => tokens.push(token::new(token::Type::Semicolon, ";", None)),
            '/' => {
                if next_char_match('/', &mut chars) {
                    comment::scan(&mut chars);
                } else {
                    tokens.push(token::new(token::Type::Slash, "/", None));
                }
            }
            '!' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(token::new(token::Type::BangEqual, "!=", None));
                } else {
                    tokens.push(token::new(token::Type::Bang, "!", None));
                }
            }
            '=' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(token::new(token::Type::EqualEqual, "==", None));
                } else {
                    tokens.push(token::new(token::Type::Equal, "=", None));
                }
            }
            '<' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(token::new(token::Type::LessEqual, "<=", None));
                } else {
                    tokens.push(token::new(token::Type::Less, "<", None));
                }
            }
            '>' => {
                if next_char_match('=', &mut chars) {
                    tokens.push(token::new(token::Type::GreaterEqual, ">=", None));
                } else {
                    tokens.push(token::new(token::Type::Greater, ">", None));
                }
            }
            '"' => match string::scan(&mut chars, &mut line) {
                Ok((text, value)) => {
                    tokens.push(token::new(token::Type::String, text.as_str(), Some(value)));
                }
                Err(error) => errors.push(error),
            },
            '0'..='9' => {
                let (text, value) = number::scan(char, &mut chars);
                tokens.push(token::new(token::Type::Number, text.as_str(), Some(value)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let text = identifier::scan(char, &mut chars);
                let keyword = keywords::map().get(text.as_str());

                if let Some(token_type) = keyword {
                    tokens.push(token::new(token_type.clone(), text.as_str(), None));
                } else {
                    tokens.push(token::new(token::Type::Identifier, text.as_str(), None));
                }
            }
            ' ' | '\t' => {}
            '\n' => {
                line += 1;
            }
            _ => {
                errors.push(token::Error {
                    message: format!("Unexpected character: {char}"),
                    line,
                });
            }
        }
    }

    for error in &errors {
        eprintln!("[line {}] Error: {}", error.line, error.message);
    }

    for token in tokens {
        let value = token.literal.clone().unwrap_or("null".to_string());
        println!("{} {} {}", token.r#type, token.lexeme, value);
    }

    println!("EOF  null");

    if !errors.is_empty() {
        std::process::exit(65);
    }
}
