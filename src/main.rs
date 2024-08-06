use std::env;
use std::fs;

mod parser;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {filename}");
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let scan_tokens = scanner::scan_tokens(&file_contents);

            for error in &scan_tokens.errors {
                eprintln!("[line {}] Error: {}", error.line, error.message);
            }

            for token in scan_tokens.tokens {
                let value = token.literal.unwrap_or("null".to_string());
                println!("{} {} {}", token.r#type, token.lexeme, value);
            }

            println!("EOF  null");

            if !scan_tokens.errors.is_empty() {
                std::process::exit(65);
            }
        }
        "parse" => {
            let scan_tokens = scanner::scan_tokens(&file_contents);
            let tree = parser::parse_tokens(&mut scan_tokens.tokens.iter().peekable());

            println!("{tree}");
        }
        _ => {
            eprintln!("Unknown command: {command}");
        }
    }
}
