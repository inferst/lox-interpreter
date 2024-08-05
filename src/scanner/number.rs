use std::{iter::Peekable, str::Chars};

pub fn scan(char: char, chars: &mut Peekable<Chars>) -> (String, String) {
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
