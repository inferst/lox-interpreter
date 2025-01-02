use std::{cell::RefCell, collections::HashMap, rc::Rc, time::SystemTime};

use super::{value::Value, Expr};

#[allow(clippy::needless_pass_by_value)]
fn clock(_args: Vec<Expr>, _scope: Scope, _args_scope: Scope) -> Value {
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    Value::Literal(super::Literal::Number(duration.as_secs_f64().floor()))
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub prev: Option<Rc<RefCell<Scope>>>,
    variables: Rc<RefCell<HashMap<String, Value>>>,
}

impl Scope {
    pub fn new(variables: HashMap<String, Value>, prev: Option<Rc<RefCell<Scope>>>) -> Self {
        Self { prev, variables: Rc::new(RefCell::new(variables)) }
    }

    pub fn global() -> Self {
        let scope = Scope::new(HashMap::new(), None);
        scope.define(String::from("clock"), Value::Callable(Rc::new(clock), None));
        scope
    }

    //pub fn push(&self) {
    //    let mut stack = self.stack.borrow_mut();
    //    stack.push(HashMap::new());
    //}
    //
    //pub fn pop(&self) {
    //    let mut stack = self.stack.borrow_mut();
    //    stack.pop();
    //}
    //
    //pub fn clear(&self) {
    //    let mut stack = self.stack.borrow_mut();
    //    stack.clear();
    //}

    pub fn define(&self, name: String, value: Value) {
        self.variables.borrow_mut().insert(name, value);
    }

    pub fn set(&self, name: &str, value: Value) {
        let mut variables = self.variables.borrow_mut();
        if variables.contains_key(name) {
            variables.insert(name.to_string(), value);
            return;
        }

        if let Some(scope) = &self.prev {
            return scope.borrow_mut().set(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Value {
        let variables = self.variables.borrow();
        if variables.contains_key(name) {
            return variables[name].clone();
        }

        if let Some(scope) = &self.prev {
            return scope.borrow().get(name);
        }

        eprintln!("Couldn't find scope value '{name}'");
        std::process::exit(70);
    }
}
