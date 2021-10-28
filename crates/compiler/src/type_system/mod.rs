pub mod infer_result;
pub mod r#type;

use std::collections::HashMap;

use self::{
  infer_result::InferResult,
  r#type::{BaseType, Type},
};

pub struct TypeSystem<'a> {
  pub variables: HashMap<&'a str, Type>,
}

impl<'a> TypeSystem<'a> {
  pub fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }
  pub fn lookup(&self, name: &'a str) -> Option<&Type> {
    self.variables.get(name)
  }
  pub fn set(&mut self, name: &'a str, ty: Type) {
    self.variables.insert(name, ty);
  }
  pub fn infer(&self, lhs: Type, rhs: Type) -> Result<InferResult, String> {
    match (BaseType::from(lhs), BaseType::from(rhs)) {
      (BaseType::Float, BaseType::Float) => Ok({
        if lhs.size() != rhs.size() {
          if lhs.size() > rhs.size() {
            InferResult::Cast(lhs)
          } else {
            InferResult::Cast(rhs)
          }
        } else {
          InferResult::Success(rhs)
        }
      }),
      (BaseType::Integer, BaseType::Integer) => Ok({
        if lhs.size() != rhs.size() {
          if lhs.size() > rhs.size() {
            InferResult::Cast(lhs)
          } else {
            InferResult::Cast(rhs)
          }
        } else {
          InferResult::Success(rhs)
        }
      }),
      _ => Err(format!("Incompatible data types, `{}` is not compatible with `{}`", lhs.to_string(), rhs.to_string()))
    }
  }
}
