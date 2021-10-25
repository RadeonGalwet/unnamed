use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::variable::Variable;

#[derive(Clone, Debug)]
pub struct Environment<'a> {
  pub store: HashMap<&'a str, Variable<'a>>,
  pub parent: Option<Rc<RefCell<Environment<'a>>>>,
}

impl<'a> Environment<'a> {
  pub fn new(parent: Option<Rc<RefCell<Environment<'a>>>>) -> Self {
    Self {
      store: HashMap::new(),
      parent,
    }
  }
  pub fn set(&mut self, key: &'a str, value: Variable<'a>) {
    self.store.insert(key, value);
  }
  pub fn clear(&mut self) {
    self.store.clear();
  }
  pub fn get(&self, key: &'a str) -> Result<Variable<'a>, String> {
    match self.store.get(key) {
      Some(value) => Ok(*value),
      None => match self.parent {
        Some(ref parent) => parent.borrow().get(key),
        None => Err("Unknown value".to_string()),
      },
    }
  }
}
