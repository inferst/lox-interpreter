use core::fmt;

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
