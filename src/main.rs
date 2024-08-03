use std::env;
use std::fs;

use token::Token;
use token::Type;

mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    if "tokenize" == command.as_str() {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file {filename}");
            String::new()
        });

        // Uncomment this block to pass the first stage
        tokenize(&file_contents);
    } else {
        eprintln!("Unknown command: {command}");
    }
}

fn tokenize(content: &str) {
    let chars = content.chars();
    let mut tokens = vec![];

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
            _other => {}
        }
    }

    for token in tokens {
        println!("{} {} null", token.r#type, token.text);
    }

    println!("EOF  null");
}
