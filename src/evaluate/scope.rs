use std::{cell::RefCell, collections::HashMap, rc::Rc, time::SystemTime};

use super::{value::Value, Expr};

#[allow(clippy::needless_pass_by_value)]
fn clock(_args: Vec<Expr>, _scope: Scope) -> Value {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    Value::Literal(super::Literal::Number(duration.as_secs_f64().floor()))
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub prev: Option<Rc<RefCell<Scope>>>,
    pub stack: Rc<RefCell<Vec<HashMap<String, Value>>>>,
}

impl Scope {
    pub fn new(stack: Vec<HashMap<String, Value>>, prev: Option<Rc<RefCell<Scope>>>) -> Self {
        let stack = Rc::new(RefCell::new(stack));
        Self { prev, stack }
    }

    pub fn global() -> Self {
        let scope = Scope::new(vec![], None);
        scope.define(String::from("clock"), Value::Callable(Rc::new(clock), None));
        scope
    }

    pub fn push(&self) {
        let mut stack = self.stack.borrow_mut();
        stack.push(HashMap::new());
    }

    pub fn pop(&self) {
        let mut stack = self.stack.borrow_mut();
        stack.pop();
    }

    pub fn clear(&self) {
        let mut stack = self.stack.borrow_mut();
        stack.clear();
    }

    pub fn define(&self, name: String, value: Value) {
        let mut stack = self.stack.borrow_mut();
        if stack.is_empty() {
            stack.push(HashMap::new());
        }
        stack.last_mut().unwrap().insert(name, value);
    }

    pub fn set(&self, name: &str, value: Value) {
        let mut stack = self.stack.borrow_mut();
        for variables in stack.iter_mut().rev() {
            if variables.contains_key(name) {
                variables.insert(name.to_string(), value);
                return;
            }
        }

        if stack.is_empty() {
            stack.push(HashMap::new());
        }
        stack.first_mut().unwrap().insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Value {
        let stack = self.stack.borrow();
        for variable in stack.iter().rev() {
            if variable.contains_key(name) {
                return variable[name].clone();
            }
        }

        if let Some(scope) = &self.prev {
            return scope.borrow().get(name);
        }

        eprintln!("Couldn't find scope value");
        std::process::exit(70);
    }
}
