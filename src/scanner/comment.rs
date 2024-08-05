use std::{iter::Peekable, str::Chars};

pub fn scan(chars: &mut Peekable<Chars>) {
    while let Some(next) = chars.peek() {
        if *next == '\n' {
            break;
        }

        chars.next();
    }
}
