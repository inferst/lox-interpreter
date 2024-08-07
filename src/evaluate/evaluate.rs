use core::fmt;

use crate::parser::{Expr, UnaryOperator};

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
            let expr = expr.as_ref();
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
        Expr::Binary(_, _, _) => todo!(),
        Expr::Grouping(expr) => evaluate(expr),
    }
}
