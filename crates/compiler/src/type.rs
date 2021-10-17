use crate::value::Value;

enum BaseType {
  Float,
  Integer,
}

impl<'a> From<Value<'a>> for BaseType {
  fn from(value: Value<'a>) -> Self {
    match value {
      Value::I16(_) | Value::I32(_) | Value::I64(_) | Value::I128(_) => Self::Integer,
      Value::F16(_) | Value::F32(_) | Value::F64(_) | Value::F128(_) => Self::Float,
    }
  }
}
