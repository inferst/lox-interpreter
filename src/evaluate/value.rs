use core::fmt;

use crate::parser::Expr;

use super::Literal;

pub type Callable = fn(Vec<Value>) -> Expr;

#[derive(Clone)]
pub enum Value {
    Value(Literal),
    Callable(String, Callable),
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Value(literal) => write!(fmt, "{literal}"),
            Value::Callable(name, _) => write!(fmt, "ƒ {name}() {{ [native code] }}"),
        }
    }
}
