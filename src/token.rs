use core::fmt;

pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens { tokens: vec![] }
    }

    pub fn add(&mut self, token_type: Type, text: String, value: Option<String>) {
        self.tokens.push(Token {
            token_type,
            text,
            value,
        });
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

pub struct TokenError {
    pub line: usize,
    pub text: String,
}

pub struct Token {
    pub token_type: Type,
    pub text: String,
    pub value: Option<String>,
}

#[derive(Clone)]
pub enum Type {
    // Single-character tokens
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

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    String,
    Number,
    Identifier,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[allow(clippy::too_many_lines)]
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
            Type::String => {
                fmt.write_str("STRING").unwrap();
            }
            Type::Number => {
                fmt.write_str("NUMBER").unwrap();
            }
            Type::Identifier => {
                fmt.write_str("IDENTIFIER").unwrap();
            }
            Type::And => {
                fmt.write_str("AND").unwrap();
            }
            Type::Class => {
                fmt.write_str("CLASS").unwrap();
            }
            Type::Else => {
                fmt.write_str("ELSE").unwrap();
            }
            Type::False => {
                fmt.write_str("FALSE").unwrap();
            }
            Type::For => {
                fmt.write_str("FOR").unwrap();
            }
            Type::Fun => {
                fmt.write_str("FUN").unwrap();
            }
            Type::If => {
                fmt.write_str("IF").unwrap();
            }
            Type::Nil => {
                fmt.write_str("NIL").unwrap();
            }
            Type::Or => {
                fmt.write_str("OR").unwrap();
            }
            Type::Print => {
                fmt.write_str("PRINT").unwrap();
            }
            Type::Return => {
                fmt.write_str("RETURN").unwrap();
            }
            Type::Super => {
                fmt.write_str("SUPER").unwrap();
            }
            Type::This => {
                fmt.write_str("THIS").unwrap();
            }
            Type::True => {
                fmt.write_str("TRUE").unwrap();
            }
            Type::Var => {
                fmt.write_str("VAR").unwrap();
            }
            Type::While => {
                fmt.write_str("WHILE").unwrap();
            }
        }
        Ok(())
    }
}
