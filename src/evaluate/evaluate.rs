use core::fmt;

use crate::parser::{BinaryOperator, Expr, UnaryOperator};

pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Boolean(bool) => write!(fmt, "{bool}"),
            Literal::Number(number) => write!(fmt, "{number}"),
            Literal::String(string) => write!(fmt, "{string}"),
            Literal::Nil => write!(fmt, "nil"),
        }
    }
}

pub fn evaluate(expr: &Expr) -> Literal {
    match expr {
        Expr::True => Literal::Boolean(true),
        Expr::False => Literal::Boolean(false),
        Expr::Nil => Literal::Nil,
        Expr::String(string) => Literal::String(string.clone()),
        Expr::Number(number) => Literal::Number(*number),
        Expr::Unary(operator, expr) => {
            let literal = evaluate(expr);
            match operator {
                UnaryOperator::Bang => match literal {
                    Literal::Boolean(bool) => Literal::Boolean(!bool),
                    Literal::Number(number) => Literal::Boolean(number == 0.0),
                    Literal::String(string) => Literal::Boolean(string.is_empty()),
                    Literal::Nil => Literal::Boolean(true),
                },
                UnaryOperator::Minus => match literal {
                    Literal::Number(number) => Literal::Number(-number),
                    _ => panic!("aboba"),
                },
            }
        }
        Expr::Binary(operator, left, right) => {
            let left = evaluate(left);
            let right = evaluate(right);

            match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => match *operator {
                    BinaryOperator::Star => Literal::Number(left * right),
                    BinaryOperator::Slash => Literal::Number(left / right),
                    BinaryOperator::Plus => Literal::Number(left + right),
                    BinaryOperator::Minus => Literal::Number(left - right),
                    _ => todo!(),
                },
                (Literal::String(left), Literal::String(right)) => match *operator {
                    BinaryOperator::Plus => Literal::String(format!("{left}{right}")),
                    _ => todo!(),
                },
                _ => panic!("aboba"),
            }
        }
        Expr::Grouping(expr) => evaluate(expr),
    }
}
