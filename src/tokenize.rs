use std::{iter::Peekable, str::Chars};

use crate::token::{Invalid, Tokens, Type};

pub fn exact_match(exact: &str, chars: &mut Peekable<Chars>) -> usize {
    let mut found = String::new();
    let len = exact.len();

    while let Some(next) = chars.peek() {
        if found.len() >= len {
            break;
        }

        found.push(*next);
    }

    if found.eq(exact) {
        found.len()
    } else {
        0
    }
}

pub fn tokenize(content: &str) {
    let mut chars = content.chars().peekable();
    let mut tokens = Tokens::new();
    let mut invalid_tokens = vec![];

    let mut line = 1;
    let mut offset = 0;

    while let Some(char) = chars.nth(offset) {
        offset = 0;

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
            '/' => tokens.add(Type::Slash, char.to_string()),
            '!' => {
                offset = exact_match("=", &mut chars);

                if offset > 0 {
                    tokens.add(Type::BangEqual, "!=".to_string());
                } else {
                    tokens.add(Type::Bang, "!".to_string());
                }
            }
            '=' => {
                offset = exact_match("=", &mut chars);

                if offset > 0 {
                    tokens.add(Type::EqualEqual, "==".to_string());
                } else {
                    tokens.add(Type::Equal, "=".to_string());
                }
            }
            '<' => {
                offset = exact_match("=", &mut chars);

                if offset > 0 {
                    tokens.add(Type::LessEqual, "<=".to_string());
                } else {
                    tokens.add(Type::Less, "<".to_string());
                }
            }
            '>' => {
                offset = exact_match("=", &mut chars);

                if offset > 0 {
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
