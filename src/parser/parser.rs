use crate::scanner::{Token, Type};

pub fn parse_tokens(tokens: Vec<Token>) {
    for token in tokens {
        match token.r#type {
            Type::True => {
                println!("aboba");
            }
            _ => {
                println!("boba");
            }
        }
    }
}
