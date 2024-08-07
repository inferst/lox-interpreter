use crate::parser::Expr;

pub fn evaluate(expr: &Expr) -> String {
    match expr {
        Expr::True => "true".to_string(),
        Expr::False => "false".to_string(),
        Expr::Nil => "nil".to_string(),
        Expr::String(_) => todo!(),
        Expr::Number(_) => todo!(),
        Expr::Unary(_, _) => todo!(),
        Expr::Binary(_, _, _) => todo!(),
        Expr::Grouping(_) => todo!(),
    }
}
