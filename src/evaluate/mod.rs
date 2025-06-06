mod literal;
mod scope;
mod value;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub(crate) use literal::Literal;
pub(crate) use scope::Scope;
pub(crate) use value::Value;

use crate::parser::{BinaryOperator, Expr, UnaryOperator};

fn value_to_literal(value: &Value) -> &Literal {
    match value {
        Value::Literal(value) => value,
        Value::Return(value) => value_to_literal(value),
        Value::Callable(_, _) => {
            eprintln!("Error: {value} is not literal");
            std::process::exit(65);
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn evaluate(expr: &Expr, scope: &Scope) -> Value {
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
        Expr::Callable(name, args) => {
            if let Value::Callable(callable, function_scope) = scope.get(name) {
                let mut callable = callable;
                let mut value = Value::Literal(Literal::Nil);

                let mut callable_scope = scope.clone();

                if let Some(function_scope) = function_scope {
                    callable_scope = function_scope;
                }

                for args in args {
                    value = callable(args.clone(), callable_scope.clone(), scope.clone());

                    if let Value::Callable(closure, scope) = value.clone() {
                        callable = closure;

                        if let Some(scope) = scope {
                            callable_scope = scope;
                        }
                    }
                }

                value
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
            println!("{result}");
            Value::Literal(Literal::Nil)
        }
        Expr::Statements(exprs) => {
            let mut statement = Value::Literal(Literal::Nil);
            let scope = Scope::new(HashMap::new(), Some(Rc::new(RefCell::new(scope.clone()))));

            for expr in exprs {
                statement = evaluate(expr, &scope);

                if let Value::Return(value) = &statement {
                    if let Value::Callable(callable, _) = &**value {
                        return Value::Callable(callable.clone(), Some(scope.clone()));
                    }

                    return statement;
                }
            }

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
                let statement = evaluate(expr2, scope);
                if let Value::Return(_) = &statement {
                    return statement;
                }
            }

            Value::Literal(Literal::Nil)
        }
        Expr::For(expr1, expr2, expr3, expr4) => {
            if let Some(expr1) = expr1 {
                evaluate(expr1, scope);
            }

            if let Some(expr2) = expr2 {
                while value_to_literal(&evaluate(expr2, scope)).as_bool() {
                    let statement = evaluate(expr4, scope);
                    if let Value::Return(_) = &statement {
                        return statement;
                    }

                    if let Some(expr3) = expr3 {
                        evaluate(expr3, scope);
                    }
                }
            }

            Value::Literal(Literal::Nil)
        }
        Expr::Fun(name, args, expr) => {
            let expr = expr.as_ref().clone();
            let expr = RefCell::new(expr);
            let args = args.clone();

            let closure = move |values: Vec<Expr>, function_scope: Scope, args_scope: Scope| {
                let args = args.clone();

                if values.len() != args.len() {
                    eprintln!(
                        "Error: Expected {} arguments but got {}",
                        args.len(),
                        values.len()
                    );
                    std::process::exit(70);
                }

                let function_scope = Scope::new(
                    HashMap::new(),
                    Some(Rc::new(RefCell::new(function_scope.clone()))),
                );

                let expr = expr.borrow();

                for (index, arg) in args.iter().enumerate() {
                    let value_expr = values.get(index).unwrap();
                    let value = evaluate(value_expr, &args_scope);
                    function_scope.define(arg.clone(), value);
                }

                let value = evaluate(&expr, &function_scope);

                match value {
                    Value::Return(value) => *value,
                    value => value,
                }
            };

            let closure = Rc::new(closure);

            scope.define(name.clone(), Value::Callable(closure, Some(scope.clone())));
            Value::Literal(Literal::Nil)
        }
        Expr::Return(expr) => {
            let value = evaluate(expr, scope);
            Value::Return(Box::new(value))
        }
    }
}
