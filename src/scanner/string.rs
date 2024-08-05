use std::{iter::Peekable, str::Chars};

use super::token;

pub fn scan(
    chars: &mut Peekable<Chars>,
    line: &mut usize,
) -> Result<(String, String), token::Error> {
    let mut value = String::new();

    while let Some(next) = chars.next() {
        if next == '"' {
            break;
        }

        value.push(next);

        if next == '\n' {
            *line += 1;
        }

        if chars.peek().is_none() {
            return Err(token::Error {
                message: "Unterminated string.".to_string(),
                line: *line,
            });
        }
    }

    Ok((format!("\"{value}\""), value))
}
