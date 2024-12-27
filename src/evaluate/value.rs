use core::fmt;
use std::rc::Rc;

use super::{Expr, Literal, Scope};

pub type Callable = Rc<dyn Fn(Vec<Expr>, Scope) -> Value>;

#[derive(Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Callable),
    Return(Box<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Literal(literal) => write!(fmt, "{literal}"),
            Value::Return(literal) => write!(fmt, "return {literal}"),
            Value::Callable(_) => write!(fmt, "<fn foo>"),
        }
    }
}
