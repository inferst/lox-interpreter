use std::{iter::Peekable, str::Chars};

use crate::token::{Invalid, Tokens, Type};

fn next_match(char: char, chars: &mut Peekable<Chars>) -> bool {
    if chars.next_if_eq(&char).is_some() {
        return true;
    }

    false
}

pub fn tokenize(content: &str) {
    let mut chars = content.chars().peekable();
    let mut tokens = Tokens::new();
    let mut invalid_tokens = vec![];

    let mut line = 1;

    while let Some(char) = chars.next() {
        match char {
            ')' => tokens.add(Type::RightParen, char.to_string()),
            '(' => tokens.add(Type::LeftParen, char.to_string()),
            '}' => tokens.add(Type::RightBrace, char.to_string()),
            '{' => tokens.add(Type::LeftBrace, char.to_string()),
            '*' => tokens.add(Type::Star, char.to_string()),
            '.' => tokens.add(Type::Dot, char.to_string()),
            ',' => tokens.add(Type::Comma, char.to_string()),
            '+' => tokens.add(Type::Plus, char.to_string()),
            '-' => tokens.add(Type::Minus, char.to_string()),
            ';' => tokens.add(Type::Semicolon, char.to_string()),
            '/' => {
                if next_match('/', &mut chars) {
                    for val in &mut chars.by_ref() {
                        if val == '\n' {
                            break;
                        }
                    }
                } else {
                    tokens.add(Type::Slash, char.to_string());
                }
            }
            '!' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::BangEqual, "!=".to_string());
                } else {
                    tokens.add(Type::Bang, "!".to_string());
                }
            }
            '=' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::EqualEqual, "==".to_string());
                } else {
                    tokens.add(Type::Equal, "=".to_string());
                }
            }
            '<' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::LessEqual, "<=".to_string());
                } else {
                    tokens.add(Type::Less, "<".to_string());
                }
            }
            '>' => {
                if next_match('=', &mut chars) {
                    tokens.add(Type::GreaterEqual, ">=".to_string());
                } else {
                    tokens.add(Type::Greater, ">".to_string());
                }
            }
            '\n' => {
                line += 1;
            }
            _other => {
                invalid_tokens.push(Invalid {
                    text: char.to_string(),
                    line,
                });
            }
        }
    }

    for invalid_token in &invalid_tokens {
        eprintln!(
            "[line {}] Error: Unexpected character: {}",
            invalid_token.line, invalid_token.text
        );
    }

    for token in tokens.tokens() {
        println!("{} {} null", token.r#type, token.text);
    }

    println!("EOF  null");

    if !invalid_tokens.is_empty() {
        std::process::exit(65);
    }
}
