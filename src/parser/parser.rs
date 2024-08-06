use core::fmt;

use crate::scanner::{Token, Type};

pub enum UnaryOperator {
    Bang,
    Minus,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bang => write!(fmt, "!"),
            Self::Minus => write!(fmt, "-"),
        }
    }
}

pub enum BinaryOperator {
    BangEqual,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Star,
    Plus,
    Minus,
    Slash,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BangEqual => write!(fmt, "!="),
            Self::EqualEqual => write!(fmt, "=="),
            Self::Less => write!(fmt, "<"),
            Self::LessEqual => write!(fmt, "<="),
            Self::Greater => write!(fmt, ">"),
            Self::GreaterEqual => write!(fmt, ">="),
            Self::Star => write!(fmt, "*"),
            Self::Plus => write!(fmt, "+"),
            Self::Minus => write!(fmt, "-"),
            Self::Slash => write!(fmt, "/"),
        }
    }
}

pub enum Expr {
    True,
    False,
    Nil,
    String(String),
    Number(f64),
    Unary(UnaryOperator, Box<Expr>),
    Binary(Box<Expr>, BinaryOperator, Box<Expr>),
    Operator(BinaryOperator),
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::True => write!(fmt, "true"),
            Self::False => write!(fmt, "false"),
            Self::Nil => write!(fmt, "nil"),
            Self::Number(number) => {
                let mut value = number.to_string();

                if !value.contains('.') {
                    value.push_str(".0");
                }

                write!(fmt, "{value}")
            }
            Self::String(string) => write!(fmt, "{string}"),
            Self::Grouping(expr) => write!(fmt, "(group {expr})"),
            _ => write!(fmt, "aboba"),
        }
    }
}

fn check_token_type(token: Option<&Token>, token_type: &Type) -> bool {
    if let Some(next) = token {
        if next.r#type == *token_type {
            return true;
        }
    }
    false
}

pub fn parse_tokens<'a, I>(tokens: &mut I) -> Expr
where
    I: Iterator<Item = &'a Token>,
{
    if let Some(token) = tokens.next() {
        return match token.r#type {
            Type::True => Expr::True,
            Type::False => Expr::False,
            Type::Nil => Expr::Nil,
            Type::Number => {
                let value = token.lexeme.parse::<f64>().unwrap();
                return Expr::Number(value);
            }
            Type::String => {
                let string = &token.literal;

                if let Some(string) = string {
                    return Expr::String(string.to_string());
                }

                return Expr::String(String::new());
            }
            Type::LeftParen => {
                let expr = parse_tokens(tokens);
                let next = tokens.next();

                if !check_token_type(next, &Type::RightParen) {
                    eprintln!("Error: Unmatched parentheses.");
                    std::process::exit(65);
                }

                return Expr::Grouping(Box::new(expr));
            }
            _ => {
                eprintln!("Error: Unmatched parentheses.");
                std::process::exit(65);
            }
        };
    }

    eprintln!("Error: Unmatched parentheses.");
    std::process::exit(65);
}
