use core::fmt;

use crate::{scanner::Type, utils::pad_number};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum Expr {
    True,
    False,
    Nil,
    String(String),
    Number(f64),
    Unary(UnaryOperator, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Grouping(Box<Expr>),
    Identifier(String),
    Assignment(String, Box<Expr>, bool),
    Print(Box<Expr>),
    Semicolon,
    Statements(Vec<Expr>),
    IfElse(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    While(Box<Expr>, Box<Expr>),
    For(
        Option<Box<Expr>>,
        Option<Box<Expr>>,
        Option<Box<Expr>>,
        Box<Expr>,
    ),
    Callable(String, Vec<Vec<Expr>>),
    Fun(String, Vec<String>, Box<Expr>),
    Return(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::True => write!(fmt, "true"),
            Self::False => write!(fmt, "false"),
            Self::Nil => write!(fmt, "nil"),
            Self::Number(number) => write!(fmt, "{}", pad_number(*number)),
            Self::String(string) => write!(fmt, "{string}"),
            Self::Grouping(expr) => write!(fmt, "(group {expr})"),
            Self::Unary(operator, expr) => write!(fmt, "({operator} {expr})"),
            Self::Binary(operator, expr1, expr2) => write!(fmt, "({operator} {expr1} {expr2})"),
            Self::Identifier(string) => write!(fmt, "identifier {string}"),
            Self::Assignment(name, expr, _) => write!(fmt, "assignment {name} = {expr}"),
            Self::Print(expr) => write!(fmt, "print {expr}"),
            Self::Semicolon => write!(fmt, "semicolon"),
            Self::Statements(exprs) => {
                let len = exprs.len();
                write!(fmt, "statements {len}")
            }
            Self::IfElse(expr1, expr2, _) => {
                write!(fmt, "if {expr1} then {expr2}")
            }
            Self::Or(left, right) => {
                write!(fmt, "{left} or {right}")
            }
            Self::And(left, right) => {
                write!(fmt, "{left} and {right}")
            }
            Self::While(expr1, expr2) => {
                write!(fmt, "while ({expr1}) {expr2}")
            }
            Self::For(expr1, expr2, expr3, expr4) => {
                write!(
                    fmt,
                    "for ({};{};{}) {}",
                    expr1.as_ref().unwrap(),
                    expr2.as_ref().unwrap(),
                    expr3.as_ref().unwrap(),
                    expr4
                )
            }
            Self::Callable(name, _args) => {
                write!(fmt, "callbale {name}")
            }
            Self::Fun(name, _args, _expr) => {
                write!(fmt, "fun {name}")
            }
            Self::Return(_expr) => {
                write!(fmt, "return")
            }
        }
    }
}
