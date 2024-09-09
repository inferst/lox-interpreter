use core::fmt;
use std::collections::HashMap;

use crate::parser::{BinaryOperator, Expr, UnaryOperator};

#[derive(Clone, Debug)]
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

pub fn evaluate(expr: &Expr, variables: &mut HashMap<String, Literal>) -> Literal {
    match expr {
        Expr::True => Literal::Boolean(true),
        Expr::False => Literal::Boolean(false),
        Expr::Nil | Expr::Semicolon => Literal::Nil,
        Expr::String(string) => Literal::String(string.clone()),
        Expr::Number(number) => Literal::Number(*number),
        Expr::Unary(operator, expr) => {
            let literal = evaluate(expr, variables);
            match operator {
                UnaryOperator::Bang => match literal {
                    Literal::Boolean(bool) => Literal::Boolean(!bool),
                    Literal::Number(number) => Literal::Boolean(number == 0.0),
                    Literal::String(string) => Literal::Boolean(string.is_empty()),
                    Literal::Nil => Literal::Boolean(true),
                },
                UnaryOperator::Minus => match literal {
                    Literal::Number(number) => Literal::Number(-number),
                    _ => std::process::exit(70),
                },
            }
        }
        Expr::Binary(operator, left, right) => {
            let left = evaluate(left, variables);
            let right = evaluate(right, variables);

            match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => match *operator {
                    BinaryOperator::Star => Literal::Number(left * right),
                    BinaryOperator::Slash => Literal::Number(left / right),
                    BinaryOperator::Plus => Literal::Number(left + right),
                    BinaryOperator::Minus => Literal::Number(left - right),
                    BinaryOperator::Greater => Literal::Boolean(left > right),
                    BinaryOperator::GreaterEqual => Literal::Boolean(left >= right),
                    BinaryOperator::Less => Literal::Boolean(left < right),
                    BinaryOperator::LessEqual => Literal::Boolean(left <= right),
                    BinaryOperator::EqualEqual => Literal::Boolean(left.eq(&right)),
                    BinaryOperator::BangEqual => Literal::Boolean(left.ne(&right)),
                },
                (Literal::String(left), Literal::String(right)) => match *operator {
                    BinaryOperator::Plus => Literal::String(format!("{left}{right}")),
                    BinaryOperator::EqualEqual => Literal::Boolean(left == right),
                    BinaryOperator::BangEqual => Literal::Boolean(left != right),
                    _ => std::process::exit(70),
                },
                (Literal::Number(_), Literal::String(_))
                | (Literal::String(_), Literal::Number(_)) => match *operator {
                    BinaryOperator::EqualEqual | BinaryOperator::BangEqual => {
                        Literal::Boolean(false)
                    }
                    _ => std::process::exit(70),
                },
                (Literal::Boolean(left), Literal::Boolean(right)) => match *operator {
                    BinaryOperator::EqualEqual => Literal::Boolean(left == right),
                    BinaryOperator::BangEqual => Literal::Boolean(left != right),
                    _ => std::process::exit(70),
                },
                _ => std::process::exit(70),
            }
        }
        Expr::Grouping(expr) => evaluate(expr, variables),
        Expr::Identifier(name) => {
            if let Some(value) = variables.get(name) {
                value.clone()
            } else {
                std::process::exit(70);
            }
        }
        Expr::Assignment(name, expr) => {
            let expr = expr.as_ref();
            if let Expr::Identifier(expr_name) = expr {
                if let Some(value) = variables.get(expr_name) {
                    let value = value.clone();
                    variables.insert(name.clone(), value.clone());
                    value
                } else {
                    std::process::exit(70);
                }
            } else {
                let value = evaluate(expr, variables);
                variables.insert(name.clone(), value.clone());
                value
            }
        }
        Expr::Print(expr) => {
            let result = evaluate(expr, variables);
            println!("{result}");
            Literal::Nil
        }
        Expr::Statements(exprs) => {
            let mut result = Literal::Nil;
            let mut scoped_variables = HashMap::new();
            scoped_variables.extend(variables.clone());

            for expr in exprs {
                result = evaluate(expr, &mut scoped_variables);
            }
            result
        }
    }
}
