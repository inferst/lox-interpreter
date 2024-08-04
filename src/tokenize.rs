use std::{iter::Peekable, str::Chars};

use crate::token::{TokenError, Tokens, Type};

fn next_match(char: char, chars: &mut Peekable<Chars>) -> bool {
    if chars.next_if_eq(&char).is_some() {
        return true;
    }

    false
}

fn string(
    char: char,
    chars: &mut Peekable<Chars>,
    line: &mut usize,
) -> Result<(String, String), TokenError> {
    let mut text = String::from(char);
    let mut value = String::new();

    while let Some(next) = chars.next() {
        text.push(next);

        if next == '\n' {
            *line += 1;
        }

        if next == '"' {
            value = text[1..text.len() - 1].to_string();
            break;
        }

        if chars.peek().is_none() {
            return Err(TokenError {
                text: "Unterminated string.".to_string(),
                line: *line,
            });
        }
    }

    Ok((text, value))
}

fn number(char: char, chars: &mut Peekable<Chars>) -> (String, String) {
    let mut value = String::from(char);

    while let Some(peek) = chars.peek() {
        if peek.is_ascii_digit() {
            value.push(*peek);
            chars.next();
        } else {
            break;
        }
    }

    if let Some(next) = chars.peek() {
        if *next == '.' {
            let mut cloned = chars.clone();
            cloned.next();

            if let Some(peek) = cloned.peek() {
                if peek.is_ascii_digit() {
                    chars.next();
                    value.push('.');

                    while let Some(next) = chars.peek() {
                        if next.is_ascii_digit() {
                            value.push(*next);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    let text = value.clone();

    let float = value
        .parse::<f64>()
        .expect("Number token should be parsed into float");

    let mut value = float.to_string();

    if !value.contains('.') {
        value.push_str(".0");
    }

    (text, value)
}

pub fn comment(chars: &mut Peekable<Chars>) {
    while let Some(next) = chars.peek() {
        if *next == '\n' {
            break;
        }

        chars.next();
    }
}

pub fn is_alpha_numeric(char: char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
}

pub fn identifier(char: char, chars: &mut Peekable<Chars>) -> String {
    let mut value = String::from(char);

    while let Some(peek) = chars.peek() {
        if !is_alpha_numeric(*peek) {
            break;
        }

        value.push(*peek);
        chars.next();
    }

    value
}

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
                    comment(&mut chars);
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
            '"' => match string(char, &mut chars, &mut line) {
                Ok((text, value)) => tokens.add(Type::String, text, Some(value)),
                Err(error) => errors.push(error),
            },
            '0'..='9' => {
                let (text, value) = number(char, &mut chars);
                tokens.add(Type::Number, text, Some(value));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let text = identifier(char, &mut chars);
                tokens.add(Type::Identifier, text, None);
            }
            ' ' | '\t' => {}
            '\n' => {
                line += 1;
            }
            _ => {
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
