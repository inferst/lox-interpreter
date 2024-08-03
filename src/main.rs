use std::env;
use std::fs;

mod token;
mod tokenize;

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
        tokenize::tokenize(&file_contents);
    } else {
        eprintln!("Unknown command: {command}");
    }
}
