use crate::value::Value;

impl Value {
  pub fn int(&self) -> Result<i32, String> {
    match self {
        Value::Integer(int) => Ok(*int),
    }
  }
}