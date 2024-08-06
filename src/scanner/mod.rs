mod comment;
mod identifier;
mod keywords;
mod number;
mod scanner;
mod string;
mod token;

pub(crate) use scanner::scan_tokens;
pub(crate) use token::{Token, Type};
