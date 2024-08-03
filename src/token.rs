use core::fmt;

pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens { tokens: vec![] }
    }

    pub fn add(&mut self, r#type: Type, text: String) {
        self.tokens.push(Token { r#type, text });
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

pub struct Token {
    pub r#type: Type,
    pub text: String,
}

pub struct Invalid {
    pub text: String,
    pub line: usize,
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
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
            Type::Bang => {
                fmt.write_str("BANG").unwrap();
            }
            Type::BangEqual => {
                fmt.write_str("BANG_EQUAL").unwrap();
            }
            Type::Equal => {
                fmt.write_str("EQUAL").unwrap();
            }
            Type::EqualEqual => {
                fmt.write_str("EQUAL_EQUAL").unwrap();
            }
            Type::Less => {
                fmt.write_str("LESS").unwrap();
            }
            Type::LessEqual => {
                fmt.write_str("LESS_EQUAL").unwrap();
            }
            Type::Greater => {
                fmt.write_str("GREATER").unwrap();
            }
            Type::GreaterEqual => {
                fmt.write_str("GREATER_EQUAL").unwrap();
            }
        }
        Ok(())
    }
}
