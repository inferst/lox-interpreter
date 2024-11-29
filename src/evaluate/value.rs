use core::fmt;
use std::{cell::RefCell, rc::Rc};

use super::{Expr, Literal, Scope};

pub type Callable = Rc<RefCell<dyn FnMut(Vec<Expr>, &mut Scope) -> Value>>;

#[derive(Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Callable),
}

impl fmt::Display for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Literal(literal) => write!(fmt, "{literal}"),
            Value::Callable(_) => write!(fmt, "<fn foo>"),
        }
    }
}
