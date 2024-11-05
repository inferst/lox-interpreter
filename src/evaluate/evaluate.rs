use core::fmt;

use crate::parser::{BinaryOperator, Expr, UnaryOperator};

use super::scope::Scope;

#[derive(Clone, Debug)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Literal {
    pub fn as_bool(&self) -> bool {
        match self {
            Literal::Boolean(value) => *value,
            Literal::Number(value) => *value != 0.0,
            Literal::String(_) => true,
            Literal::Nil => false,
        }
    }
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

#[allow(clippy::too_many_lines)]
pub fn evaluate(expr: &Expr, scope: &mut Scope) -> Literal {
    match expr {
        Expr::True => Literal::Boolean(true),
        Expr::False => Literal::Boolean(false),
        Expr::Nil | Expr::Semicolon => Literal::Nil,
        Expr::String(string) => Literal::String(string.clone()),
        Expr::Number(number) => Literal::Number(*number),
        Expr::Unary(operator, expr) => {
            let literal = evaluate(expr, scope);
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
        Expr::Or(left, right) => {
            let left = evaluate(left, scope);
            let value = left.as_bool();

            if value {
                return left;
            }

            evaluate(right, scope)
        }
        Expr::And(left, right) => {
            let left = evaluate(left, scope);
            let value = left.as_bool();

            if !value {
                return left;
            }

            evaluate(right, scope)
        }
        Expr::Binary(operator, left, right) => {
            let left = evaluate(left, scope);
            let right = evaluate(right, scope);

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
        Expr::Grouping(expr) => evaluate(expr, scope),
        Expr::Identifier(name) => scope.get(name),
        Expr::Assignment(name, expr, define) => {
            let expr = expr.as_ref();

            if *define {
                let value = evaluate(expr, scope);
                scope.define(name.clone(), value.clone());
                value
            } else if let Expr::Identifier(expr_name) = expr {
                let value = scope.get(expr_name);
                scope.set(name, value.clone());
                value
            } else {
                let value = evaluate(expr, scope);
                scope.set(name, value.clone());
                value
            }
        }
        Expr::Print(expr) => {
            let result = evaluate(expr, scope);
            println!("{result}");
            Literal::Nil
        }
        Expr::Statements(exprs) => {
            let mut statement = Literal::Nil;
            scope.push();

            for expr in exprs {
                statement = evaluate(expr, scope);
            }

            scope.pop();
            statement
        }
        Expr::IfElse(expr1, expr2, expr3) => {
            let statement = evaluate(expr1, scope);

            if statement.as_bool() {
                return evaluate(expr2, scope);
            } else if let Some(else_expr) = expr3 {
                return evaluate(else_expr, scope);
            }

            Literal::Nil
        }
        Expr::While(expr1, expr2) => {
            while evaluate(expr1, scope).as_bool() {
                evaluate(expr2, scope);
            }

            Literal::Nil
        }
        Expr::For(expr1, expr2, expr3, expr4) => {

            //dbg!(expr1);
            //dbg!(expr2);
            //dbg!(expr3);
            //dbg!(expr4);

            if let Some(expr1) = expr1 {
                evaluate(expr1, scope);
            }

            while evaluate(expr2, scope).as_bool() {
                evaluate(expr4, scope);

                if let Some(expr3) = expr3 {
                    evaluate(expr3, scope);
                }
            }

            Literal::Nil
        }
    }
}
