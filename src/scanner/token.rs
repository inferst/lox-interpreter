use core::fmt;

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct Token {
    pub r#type: Type,
    pub lexeme: String,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(r#type: Type, text: &str, value: Option<String>) -> Token {
        Token {
            r#type,
            lexeme: text.to_string(),
            literal: value,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
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

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::RightBrace => write!(fmt, "RIGHT_BRACE"),
            Type::LeftBrace => write!(fmt, "LEFT_BRACE"),
            Type::RightParen => write!(fmt, "RIGHT_PAREN"),
            Type::LeftParen => write!(fmt, "LEFT_PAREN"),
            Type::Star => write!(fmt, "STAR"),
            Type::Dot => write!(fmt, "DOT"),
            Type::Comma => write!(fmt, "COMMA"),
            Type::Plus => write!(fmt, "PLUS"),
            Type::Minus => write!(fmt, "MINUS"),
            Type::Semicolon => write!(fmt, "SEMICOLON"),
            Type::Slash => write!(fmt, "SLASH"),
            Type::Bang => write!(fmt, "BANG"),
            Type::BangEqual => write!(fmt, "BANG_EQUAL"),
            Type::Equal => write!(fmt, "EQUAL"),
            Type::EqualEqual => write!(fmt, "EQUAL_EQUAL"),
            Type::Less => write!(fmt, "LESS"),
            Type::LessEqual => write!(fmt, "LESS_EQUAL"),
            Type::Greater => write!(fmt, "GREATER"),
            Type::GreaterEqual => write!(fmt, "GREATER_EQUAL"),
            Type::String => write!(fmt, "STRING"),
            Type::Number => write!(fmt, "NUMBER"),
            Type::Identifier => write!(fmt, "IDENTIFIER"),
            Type::And => write!(fmt, "AND"),
            Type::Class => write!(fmt, "CLASS"),
            Type::Else => write!(fmt, "ELSE"),
            Type::False => write!(fmt, "FALSE"),
            Type::For => write!(fmt, "FOR"),
            Type::Fun => write!(fmt, "FUN"),
            Type::If => write!(fmt, "IF"),
            Type::Nil => write!(fmt, "NIL"),
            Type::Or => write!(fmt, "OR"),
            Type::Print => write!(fmt, "PRINT"),
            Type::Return => write!(fmt, "RETURN"),
            Type::Super => write!(fmt, "SUPER"),
            Type::This => write!(fmt, "THIS"),
            Type::True => write!(fmt, "TRUE"),
            Type::Var => write!(fmt, "VAR"),
            Type::While => write!(fmt, "WHILE"),
        }
    }
}
