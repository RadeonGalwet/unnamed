use crate::value::Value;

enum BaseType {
  Float,
  Integer,
  Pointer,
}

impl<'a> From<Value<'a>> for BaseType {
  fn from(value: Value<'a>) -> Self {
    match value {
      Value::I16(_) | Value::I32(_) | Value::I64(_) | Value::I128(_) => Self::Integer,
      Value::F16(_) | Value::F32(_) | Value::F64(_) | Value::F128(_) => Self::Float,
      Value::Pointer(..) => Self::Pointer,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
  I16,
  I32,
  I64,
  I128,
  F16,
  F32,
  F64,
  F128,
  Pointer,
}

impl<'a> From<&Value<'a>> for Type {
  fn from(value: &Value<'a>) -> Self {
    match value {
      Value::I16(_) => Self::I16,
      Value::I32(_) => Self::I32,
      Value::I64(_) => Self::I64,
      Value::I128(_) => Self::I128,
      Value::F16(_) => Self::F16,
      Value::F32(_) => Self::F32,
      Value::F64(_) => Self::F64,
      Value::F128(_) => Self::F128,
      Value::Pointer(..) => Self::Pointer,
    }
  }
}



impl<'a> From<Value<'a>> for Type {
  fn from(value: Value<'a>) -> Self {
    match value {
      Value::I16(_) => Self::I16,
      Value::I32(_) => Self::I32,
      Value::I64(_) => Self::I64,
      Value::I128(_) => Self::I128,
      Value::F16(_) => Self::F16,
      Value::F32(_) => Self::F32,
      Value::F64(_) => Self::F64,
      Value::F128(_) => Self::F128,
      Value::Pointer(..) => Self::Pointer,
    }
  }
}

impl From<&str> for Type {
  fn from(str: &str) -> Self {
    match str {
      "i16" => Self::I16,
      "i32" => Self::I32,
      "i64" => Self::I64,
      "i128" => Self::I128,
      "f16" => Self::F16,
      "f32" => Self::F32,
      "f64" => Self::F64,
      "f128" => Self::F128,
      _ => unreachable!(),
    }
  }
}
