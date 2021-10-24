use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct Environment<'a, T> {
  pub store: HashMap<&'a str, T>,
  pub parent: Option<Rc<RefCell<Environment<'a, T>>>>,
}

impl<'a, T> Environment<'a, T> {
  pub fn new(parent: Option<Rc<RefCell<Environment<'a, T>>>>) -> Self {
    Self {
      store: HashMap::new(),
      parent,
    }
  }
  pub fn set(&mut self, key: &'a str, value: T) {
    self.store.insert(key, value);
  }
  pub fn clear(&mut self) {
    self.store.clear();
  }
  pub fn get(&self, key: &'a str) -> Result<&T, String> {
    self
      .store
      .get(key)
      .ok_or_else(|| "Unknown value".to_string())
  }
}
