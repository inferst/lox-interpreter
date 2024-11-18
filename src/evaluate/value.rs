use core::fmt;

use crate::parser::Expr;

use super::Literal;

pub type Callable = fn(Vec<Value>) -> Expr;

#[derive(Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Callable),
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Literal(literal) => write!(fmt, "{literal}"),
            Value::Callable(_) => write!(fmt, "ƒ() {{ [native code] }}"),
        }
    }
}
