mod literal;
mod scope;
mod value;

pub(crate) use literal::Literal;
pub(crate) use scope::Scope;
pub(crate) use value::Value;

use crate::parser::{BinaryOperator, Expr, UnaryOperator};

fn value_to_literal(value: &Value) -> &Literal {
    let Value::Literal(value) = value else {
        eprintln!("Error: {value} is not literal");
        std::process::exit(65);
    };

    value
}

#[allow(clippy::too_many_lines)]
pub fn evaluate(expr: &Expr, scope: &mut Scope) -> Value {
    match expr {
        Expr::True => Value::Literal(Literal::Boolean(true)),
        Expr::False => Value::Literal(Literal::Boolean(false)),
        Expr::Nil | Expr::Semicolon => Value::Literal(Literal::Nil),
        Expr::String(string) => Value::Literal(Literal::String(string.clone())),
        Expr::Number(number) => Value::Literal(Literal::Number(*number)),
        Expr::Unary(operator, expr) => {
            let value = evaluate(expr, scope);
            let literal = value_to_literal(&value);

            match operator {
                UnaryOperator::Bang => match literal {
                    Literal::Boolean(bool) => Value::Literal(Literal::Boolean(!bool)),
                    Literal::Number(number) => Value::Literal(Literal::Boolean(*number == 0.0)),
                    Literal::String(string) => Value::Literal(Literal::Boolean(string.is_empty())),
                    Literal::Nil => Value::Literal(Literal::Boolean(true)),
                },
                UnaryOperator::Minus => match literal {
                    Literal::Number(number) => Value::Literal(Literal::Number(-number)),
                    _ => std::process::exit(70),
                },
            }
        }
        Expr::Or(left, right) => {
            let left = evaluate(left, scope);
            let literal = value_to_literal(&left);
            let value = literal.as_bool();

            if value {
                return left;
            }

            evaluate(right, scope)
        }
        Expr::And(left, right) => {
            let left = evaluate(left, scope);
            let literal = value_to_literal(&left).as_bool();

            if !literal {
                return left;
            }

            evaluate(right, scope)
        }
        Expr::Binary(operator, left, right) => {
            let left = evaluate(left, scope);
            let left = value_to_literal(&left);

            let right = evaluate(right, scope);
            let right = value_to_literal(&right);

            match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => match *operator {
                    BinaryOperator::Star => Value::Literal(Literal::Number(left * right)),
                    BinaryOperator::Slash => Value::Literal(Literal::Number(left / right)),
                    BinaryOperator::Plus => Value::Literal(Literal::Number(left + right)),
                    BinaryOperator::Minus => Value::Literal(Literal::Number(left - right)),
                    BinaryOperator::Greater => Value::Literal(Literal::Boolean(left > right)),
                    BinaryOperator::GreaterEqual => Value::Literal(Literal::Boolean(left >= right)),
                    BinaryOperator::Less => Value::Literal(Literal::Boolean(left < right)),
                    BinaryOperator::LessEqual => Value::Literal(Literal::Boolean(left <= right)),
                    BinaryOperator::EqualEqual => Value::Literal(Literal::Boolean(left.eq(right))),
                    BinaryOperator::BangEqual => Value::Literal(Literal::Boolean(left.ne(right))),
                },
                (Literal::String(left), Literal::String(right)) => match *operator {
                    BinaryOperator::Plus => {
                        Value::Literal(Literal::String(format!("{left}{right}")))
                    }
                    BinaryOperator::EqualEqual => Value::Literal(Literal::Boolean(left == right)),
                    BinaryOperator::BangEqual => Value::Literal(Literal::Boolean(left != right)),
                    _ => std::process::exit(70),
                },
                (Literal::Number(_), Literal::String(_))
                | (Literal::String(_), Literal::Number(_)) => match *operator {
                    BinaryOperator::EqualEqual | BinaryOperator::BangEqual => {
                        Value::Literal(Literal::Boolean(false))
                    }
                    _ => std::process::exit(70),
                },
                (Literal::Boolean(left), Literal::Boolean(right)) => match *operator {
                    BinaryOperator::EqualEqual => Value::Literal(Literal::Boolean(left == right)),
                    BinaryOperator::BangEqual => Value::Literal(Literal::Boolean(left != right)),
                    _ => std::process::exit(70),
                },
                _ => std::process::exit(70),
            }
        }
        Expr::Grouping(expr) => evaluate(expr, scope),
        Expr::Identifier(name) => scope.get(name),
        Expr::Callable(name) => {
            if let Value::Callable(callable) = scope.get(name) {
                let expr = callable(vec![]);
                evaluate(&expr, scope)
            } else {
                eprintln!("Error: {name} is not callable");
                std::process::exit(65);
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
            let result = value_to_literal(&result);
            println!("{result}");
            Value::Literal(Literal::Nil)
        }
        Expr::Statements(exprs) => {
            let mut statement = Value::Literal(Literal::Nil);
            scope.push();

            for expr in exprs {
                statement = evaluate(expr, scope);
            }

            scope.pop();
            statement
        }
        Expr::IfElse(expr1, expr2, expr3) => {
            let statement = evaluate(expr1, scope);
            let statement = value_to_literal(&statement);

            if statement.as_bool() {
                return evaluate(expr2, scope);
            } else if let Some(else_expr) = expr3 {
                return evaluate(else_expr, scope);
            }

            Value::Literal(Literal::Nil)
        }
        Expr::While(expr1, expr2) => {
            while value_to_literal(&evaluate(expr1, scope)).as_bool() {
                evaluate(expr2, scope);
            }

            Value::Literal(Literal::Nil)
        }
        Expr::For(expr1, expr2, expr3, expr4) => {
            if let Some(expr1) = expr1 {
                evaluate(expr1, scope);
            }

            if let Some(expr2) = expr2 {
                while value_to_literal(&evaluate(expr2, scope)).as_bool() {
                    evaluate(expr4, scope);

                    if let Some(expr3) = expr3 {
                        evaluate(expr3, scope);
                    }
                }
            }

            Value::Literal(Literal::Nil)
        }
    }
}
