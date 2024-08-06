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
            _ => write!(fmt, "aboba"),
        }
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> String {
    let mut tree = String::new();

    for token in tokens {
        match token.r#type {
            Type::True => {
                tree.push_str(&format!("{}", Expr::True));
            }
            Type::False => {
                tree.push_str(&format!("{}", Expr::False));
            }
            Type::Nil => {
                tree.push_str(&format!("{}", Expr::Nil));
            }
            _ => {
                tree.push_str("aboba");
            }
        }
    }

    tree
}
