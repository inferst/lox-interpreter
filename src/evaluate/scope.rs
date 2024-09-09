use std::collections::HashMap;

use super::Literal;

pub struct Scope {
    stack: Vec<HashMap<String, Literal>>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            stack: vec![HashMap::new()],
        }
    }

    pub fn push(&mut self) {
        self.stack.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.stack.last_mut().unwrap().insert(name, value);
    }

    pub fn set(&mut self, name: &str, value: Literal) {
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

    pub fn get(&self, name: &str) -> Literal {
        for variable in self.stack.iter().rev() {
            if variable.contains_key(name) {
                return variable[name].clone();
            }
        }

        std::process::exit(70);
    }
}
