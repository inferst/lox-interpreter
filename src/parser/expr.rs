use core::fmt;

use crate::scanner::Type;

#[derive(Clone)]
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

impl From<Type> for UnaryOperator {
    fn from(value: Type) -> Self {
        match value {
            Type::Bang => UnaryOperator::Bang,
            Type::Minus => UnaryOperator::Minus,
            _ => panic!("Unknown token type"),
        }
    }
}

#[derive(Clone)]
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

impl From<Type> for BinaryOperator {
    fn from(value: Type) -> Self {
        match value {
            Type::BangEqual => BinaryOperator::BangEqual,
            Type::EqualEqual => BinaryOperator::EqualEqual,
            Type::Less => BinaryOperator::Less,
            Type::LessEqual => BinaryOperator::LessEqual,
            Type::Greater => BinaryOperator::Greater,
            Type::GreaterEqual => BinaryOperator::GreaterEqual,
            Type::Star => BinaryOperator::Star,
            Type::Plus => BinaryOperator::Plus,
            Type::Minus => BinaryOperator::Minus,
            Type::Slash => BinaryOperator::Slash,
            _ => panic!("Unknown token type"),
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    True,
    False,
    Nil,
    String(String),
    Number(f64),
    Unary(UnaryOperator, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    //Operator(BinaryOperator),
    Grouping(Box<Expr>),
}

fn pad(number: f64) -> String {
    let mut value = number.to_string();
    if !value.contains('.') {
        value.push_str(".0");
    }
    value
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::True => write!(fmt, "true"),
            Self::False => write!(fmt, "false"),
            Self::Nil => write!(fmt, "nil"),
            Self::Number(number) => write!(fmt, "{}", pad(*number)),
            Self::String(string) => write!(fmt, "{string}"),
            Self::Grouping(expr) => write!(fmt, "(group {expr})"),
            Self::Unary(operator, expr) => write!(fmt, "({operator} {expr})"),
            Self::Binary(operator, expr1, expr2) => write!(fmt, "({operator} {expr1} {expr2})"),
        }
    }
}