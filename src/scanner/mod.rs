mod scanner;
mod keywords;
mod token;
mod string;
mod number;
mod comment;
mod identifier;

pub(crate) use scanner::scan_tokens;
