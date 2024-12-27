use core::fmt;
use std::rc::Rc;

use super::{Expr, Literal, Scope};

pub type Callable = Rc<dyn Fn(Vec<Expr>, Scope) -> Value>;

#[derive(Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Callable, Option<Scope>),
    Return(Box<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Literal(literal) => write!(fmt, "{literal}"),
            Value::Return(value) => write!(fmt, "{value}"),
            Value::Callable(_, _) => write!(fmt, "<fn foo>"),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(literal) => write!(fmt, "{literal}"),
            Value::Return(literal) => write!(fmt, "{literal}"),
            Value::Callable(_, _) => write!(fmt, "<fn foo>"),
        }
    }
}
