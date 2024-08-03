use crate::token::Invalid;
use crate::token::Token;
use crate::token::Type;

pub fn tokenize(content: &str) {
    let chars = content.chars();
    let mut tokens = vec![];
    let mut invalid_tokens = vec![];

    let mut line = 1;

    for char in chars {
        match char {
            ')' => tokens.push(Token {
                r#type: Type::RightParen,
                text: char.to_string(),
            }),
            '(' => tokens.push(Token {
                r#type: Type::LeftParen,
                text: char.to_string(),
            }),
            '}' => tokens.push(Token {
                r#type: Type::RightBrace,
                text: char.to_string(),
            }),
            '{' => tokens.push(Token {
                r#type: Type::LeftBrace,
                text: char.to_string(),
            }),
            '*' => tokens.push(Token {
                r#type: Type::Star,
                text: char.to_string(),
            }),
            '.' => tokens.push(Token {
                r#type: Type::Dot,
                text: char.to_string(),
            }),
            ',' => tokens.push(Token {
                r#type: Type::Comma,
                text: char.to_string(),
            }),
            '+' => tokens.push(Token {
                r#type: Type::Plus,
                text: char.to_string(),
            }),
            '-' => tokens.push(Token {
                r#type: Type::Minus,
                text: char.to_string(),
            }),
            ';' => tokens.push(Token {
                r#type: Type::Semicolon,
                text: char.to_string(),
            }),
            '/' => tokens.push(Token {
                r#type: Type::Slash,
                text: char.to_string(),
            }),
            '\n' => {
                line += 1;
            }
            _other => invalid_tokens.push(Invalid {
                text: char.to_string(),
                line,
            }),
        }
    }

    for invalid_token in &invalid_tokens {
        eprintln!(
            "[line {}] Error: Unexpected character: {}",
            invalid_token.line, invalid_token.text
        );
    }

    for token in tokens {
        println!("{} {} null", token.r#type, token.text);
    }

    println!("EOF  null");

    if !invalid_tokens.is_empty() {
        std::process::exit(65);
    }
}
