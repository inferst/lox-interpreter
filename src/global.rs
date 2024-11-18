use std::time::SystemTime;

use crate::{
    evaluate::{Scope, Value},
    parser::Expr,
};

#[allow(clippy::needless_pass_by_value)]
fn clock(_args: Vec<Value>) -> Expr {
    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);

    if let Ok(duration) = time {
        return Expr::Number(duration.as_secs_f64().floor());
    }

    panic!();
}

pub fn define(scope: &mut Scope) {
    scope.define("clock".to_string(), Value::Callable(clock));
}
