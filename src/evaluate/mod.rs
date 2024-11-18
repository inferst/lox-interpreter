mod literal;
mod scope;
mod value;

pub(crate) use literal::Literal;
pub(crate) use scope::Scope;
pub(crate) use value::Value;

use crate::parser::{BinaryOperator, Expr, UnaryOperator};

#[allow(clippy::too_many_lines)]
pub fn evaluate(expr: &Expr, scope: &mut Scope) -> Value {
    match expr {
        Expr::True => Value::Value(Literal::Boolean(true)),
        Expr::False => Value::Value(Literal::Boolean(false)),
        Expr::Nil | Expr::Semicolon => Value::Value(Literal::Nil),
        Expr::String(string) => Value::Value(Literal::String(string.clone())),
        Expr::Number(number) => Value::Value(Literal::Number(*number)),
        Expr::Unary(operator, expr) => {
            let value = evaluate(expr, scope);
            let Value::Value(literal) = value else {
                panic!();
            };

            match operator {
                UnaryOperator::Bang => match literal {
                    Literal::Boolean(bool) => Value::Value(Literal::Boolean(!bool)),
                    Literal::Number(number) => Value::Value(Literal::Boolean(number == 0.0)),
                    Literal::String(string) => Value::Value(Literal::Boolean(string.is_empty())),
                    Literal::Nil => Value::Value(Literal::Boolean(true)),
                },
                UnaryOperator::Minus => match literal {
                    Literal::Number(number) => Value::Value(Literal::Number(-number)),
                    _ => std::process::exit(70),
                },
            }
        }
        Expr::Or(left, right) => {
            let left = evaluate(left, scope);
            let Value::Value(literal) = &left else {
                panic!();
            };
            let value = literal.as_bool();

            if value {
                return left;
            }

            evaluate(right, scope)
        }
        Expr::And(left, right) => {
            let left = evaluate(left, scope);
            let Value::Value(literal) = &left else {
                panic!();
            };
            let value = literal.as_bool();

            if !value {
                return left;
            }

            evaluate(right, scope)
        }
        Expr::Binary(operator, left, right) => {
            let left = evaluate(left, scope);
            let right = evaluate(right, scope);

            let Value::Value(left) = left else {
                panic!();
            };

            let Value::Value(right) = right else {
                panic!();
            };

            match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => match *operator {
                    BinaryOperator::Star => Value::Value(Literal::Number(left * right)),
                    BinaryOperator::Slash => Value::Value(Literal::Number(left / right)),
                    BinaryOperator::Plus => Value::Value(Literal::Number(left + right)),
                    BinaryOperator::Minus => Value::Value(Literal::Number(left - right)),
                    BinaryOperator::Greater => Value::Value(Literal::Boolean(left > right)),
                    BinaryOperator::GreaterEqual => Value::Value(Literal::Boolean(left >= right)),
                    BinaryOperator::Less => Value::Value(Literal::Boolean(left < right)),
                    BinaryOperator::LessEqual => Value::Value(Literal::Boolean(left <= right)),
                    BinaryOperator::EqualEqual => Value::Value(Literal::Boolean(left.eq(&right))),
                    BinaryOperator::BangEqual => Value::Value(Literal::Boolean(left.ne(&right))),
                },
                (Literal::String(left), Literal::String(right)) => match *operator {
                    BinaryOperator::Plus => Value::Value(Literal::String(format!("{left}{right}"))),
                    BinaryOperator::EqualEqual => Value::Value(Literal::Boolean(left == right)),
                    BinaryOperator::BangEqual => Value::Value(Literal::Boolean(left != right)),
                    _ => std::process::exit(70),
                },
                (Literal::Number(_), Literal::String(_))
                | (Literal::String(_), Literal::Number(_)) => match *operator {
                    BinaryOperator::EqualEqual | BinaryOperator::BangEqual => {
                        Value::Value(Literal::Boolean(false))
                    }
                    _ => std::process::exit(70),
                },
                (Literal::Boolean(left), Literal::Boolean(right)) => match *operator {
                    BinaryOperator::EqualEqual => Value::Value(Literal::Boolean(left == right)),
                    BinaryOperator::BangEqual => Value::Value(Literal::Boolean(left != right)),
                    _ => std::process::exit(70),
                },
                _ => std::process::exit(70),
            }
        }
        Expr::Grouping(expr) => evaluate(expr, scope),
        Expr::Identifier(name) => {
            let value = scope.get(name);
            if let Value::Value(value) = value {
                Value::Value(value)
            } else {
                panic!()
            }
        }
        Expr::Callable(name) => {
            let value = scope.get(name);
            if let Value::Callable(_, callable) = value {
                let expr = callable(vec![]);
                evaluate(&expr, scope)
            } else {
                panic!()
            }
        }
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
            let Value::Value(result) = result else {
                panic!();
            };
            println!("{result}");
            Value::Value(Literal::Nil)
        }
        Expr::Statements(exprs) => {
            let mut statement = Value::Value(Literal::Nil);
            scope.push();

            for expr in exprs {
                statement = evaluate(expr, scope);
            }

            scope.pop();
            statement
        }
        Expr::IfElse(expr1, expr2, expr3) => {
            let statement = evaluate(expr1, scope);

            if let Value::Value(statement) = statement {
                if statement.as_bool() {
                    return evaluate(expr2, scope);
                } else if let Some(else_expr) = expr3 {
                    return evaluate(else_expr, scope);
                }
            }

            Value::Value(Literal::Nil)
        }
        Expr::While(expr1, expr2) => {
            let expr = |scope: &mut Scope| {
                if let Value::Value(expr1) = evaluate(expr1, scope) {
                    return expr1.as_bool();
                }

                false
            };

            while expr(scope) {
                evaluate(expr2, scope);
            }

            Value::Value(Literal::Nil)
        }
        Expr::For(expr1, expr2, expr3, expr4) => {
            if let Some(expr1) = expr1 {
                evaluate(expr1, scope);
            }

            if let Some(expr2) = expr2 {
                let expr = |scope: &mut Scope| {
                    if let Value::Value(expr) = evaluate(expr2, scope) {
                        return expr.as_bool();
                    }

                    false
                };

                while expr(scope) {
                    evaluate(expr4, scope);

                    if let Some(expr3) = expr3 {
                        evaluate(expr3, scope);
                    }
                }
            }

            Value::Value(Literal::Nil)
        }
    }
}
