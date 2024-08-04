use std::{iter::Peekable, str::Chars};

use crate::token::{TokenError, Tokens, Type};

fn next_match(char: char, chars: &mut Peekable<Chars>) -> bool {
    if chars.next_if_eq(&char).is_some() {
        return true;
    }

    false
}

#[allow(clippy::too_many_lines)]
pub fn tokenize(content: &str) {
    let mut chars = content.chars().peekable();
    let mut tokens = Tokens::new();

    let mut errors: Vec<TokenError> = vec![];

    let mut line = 1;

    while let Some(char) = chars.next() {
        match char {
            ')' => tokens.add(Type::RightParen, char.to_string(), None),
            '(' => tokens.add(Type::LeftParen, char.to_string(), None),
            '}' => tokens.add(Type::RightBrace, char.to_string(), None),
            '{' => tokens.add(Type::LeftBrace, char.to_string(), None),
            '*' => tokens.add(Type::Star, char.to_string(), None),
            '.' => tokens.add(Type::Dot, char.to_string(), None),
            ',' => tokens.add(Type::Comma, char.to_string(), None),
            '+' => tokens.add(Type::Plus, char.to_string(), None),
            '-' => tokens.add(Type::Minus, char.to_string(), None),
            ';' => tokens.add(Type::Semicolon, char.to_string(), None),
            '/' => {
                if next_match('/', &mut chars) {
                    while let Some(next) = chars.peek() {
                        if *next == '\n' {
                            break;
                        }

                        chars.next();
                    }
                } else {
                    tokens.add(Type::Slash, char.to_string(), None);
                }
            }
            '!' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::BangEqual, "!=".to_string(), None);
                } else {
                    tokens.add(Type::Bang, "!".to_string(), None);
                }
            }
            '=' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::EqualEqual, "==".to_string(), None);
                } else {
                    tokens.add(Type::Equal, "=".to_string(), None);
                }
            }
            '<' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::LessEqual, "<=".to_string(), None);
                } else {
                    tokens.add(Type::Less, "<".to_string(), None);
                }
            }
            '>' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::GreaterEqual, ">=".to_string(), None);
                } else {
                    tokens.add(Type::Greater, ">".to_string(), None);
                }
            }
            '"' => {
                let mut str = String::from('"');

                while let Some(next) = chars.next() {
                    str.push(next);

                    if next == '\n' {
                        line += 1;
                    }

                    if next == '"' {
                        let end = str.len() - 1;
                        let value = &str.clone()[1..end];
                        tokens.add(Type::String, str, Some(value.to_string()));
                        break;
                    }

                    if chars.peek().is_none() {
                        errors.push(TokenError {
                            text: "Unterminated string.".to_string(),
                            line,
                        });
                    }
                }
            }
            ' ' | '\t' => {}
            '\n' => {
                line += 1;
            }
            _other => {
                errors.push(TokenError {
                    text: format!("Unexpected character: {char}"),
                    line,
                });
            }
        }
    }

    for error in &errors {
        eprintln!("[line {}] Error: {}", error.line, error.text);
    }

    for token in tokens.tokens() {
        let value = token.value.clone().unwrap_or("null".to_string());
        println!("{} {} {}", token.token_type, token.text, value);
    }

    println!("EOF  null");

    if !errors.is_empty() {
        std::process::exit(65);
    }
}
