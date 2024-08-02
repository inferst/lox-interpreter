use core::fmt;
use std::env;
use std::fs;

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

struct Token {
    r#type: TokenType,
    text: String,
}

enum TokenType {
    RightParen,
    RightBrace,
    LeftParen,
    LeftBrace,
}

impl fmt::Display for TokenType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::RightBrace => {
                fmt.write_str("RIGHT_BRACE").unwrap();
            }
            TokenType::LeftBrace => {
                fmt.write_str("LEFT_BRACE").unwrap();
            }
            TokenType::RightParen => {
                fmt.write_str("RIGHT_PAREN").unwrap();
            }
            TokenType::LeftParen => {
                fmt.write_str("LEFT_PAREN").unwrap();
            }
        }
        Ok(())
    }
}

fn tokenize(content: &str) {
    let chars = content.chars();
    let mut tokens = vec![];

    for char in chars {
        match char {
            ')' => tokens.push(Token {
                r#type: TokenType::RightParen,
                text: char.to_string(),
            }),
            '(' => tokens.push(Token {
                r#type: TokenType::LeftParen,
                text: char.to_string(),
            }),
            '}' => tokens.push(Token {
                r#type: TokenType::RightBrace,
                text: char.to_string(),
            }),
            '{' => tokens.push(Token {
                r#type: TokenType::LeftBrace,
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
