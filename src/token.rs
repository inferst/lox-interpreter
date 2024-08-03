use core::fmt;

pub struct Token {
    pub r#type: Type,
    pub text: String,
}

pub struct Invalid {
    pub text: String,
    pub line: u128,
}

pub enum Type {
    RightParen,
    RightBrace,
    LeftParen,
    LeftBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    Semicolon,
    Slash,
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::RightBrace => {
                fmt.write_str("RIGHT_BRACE").unwrap();
            }
            Type::LeftBrace => {
                fmt.write_str("LEFT_BRACE").unwrap();
            }
            Type::RightParen => {
                fmt.write_str("RIGHT_PAREN").unwrap();
            }
            Type::LeftParen => {
                fmt.write_str("LEFT_PAREN").unwrap();
            }
            Type::Star => {
                fmt.write_str("STAR").unwrap();
            }
            Type::Dot => {
                fmt.write_str("DOT").unwrap();
            }
            Type::Comma => {
                fmt.write_str("COMMA").unwrap();
            }
            Type::Plus => {
                fmt.write_str("PLUS").unwrap();
            }
            Type::Minus => {
                fmt.write_str("MINUS").unwrap();
            }
            Type::Semicolon => {
                fmt.write_str("SEMICOLON").unwrap();
            }
            Type::Slash => {
                fmt.write_str("SLASH").unwrap();
            }
        }
        Ok(())
    }
}
