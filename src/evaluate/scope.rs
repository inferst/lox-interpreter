use std::{cell::RefCell, collections::HashMap, rc::Rc, time::SystemTime};

use super::{value::Value, Expr};

#[allow(clippy::needless_pass_by_value)]
fn clock(_args: Vec<Expr>, _scope: &mut Scope) -> Value {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    Value::Literal(super::Literal::Number(duration.as_secs_f64().floor()))
}

#[derive(Clone)]
pub struct Scope {
    stack: Vec<HashMap<String, Value>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub fn global() -> Self {
        let mut scope = Scope::new();
        scope.define(
            String::from("clock"),
            Value::Callable(Rc::new(RefCell::new(clock))),
        );
        scope
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.stack.last_mut().unwrap().insert(name, value);
    }

    pub fn set(&mut self, name: &str, value: Value) {
        for variables in self.stack.iter_mut().rev() {
            if variables.contains_key(name) {
                variables.insert(name.to_string(), value);
                return;
            }
        }

        self.stack
            .first_mut()
            .unwrap()
            .insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Value {
        for variable in self.stack.iter().rev() {
            if variable.contains_key(name) {
                return variable[name].clone();
            }
        }

        std::process::exit(70);
    }
}
