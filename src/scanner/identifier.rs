use std::{iter::Peekable, str::Chars};

pub fn is_alpha_numeric(char: char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
}

pub fn scan(char: char, chars: &mut Peekable<Chars>) -> String {
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
