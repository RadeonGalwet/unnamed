use inkwell::{context::Context, types::BasicTypeEnum};

use crate::value::Value;

enum BaseType {
  Float,
  Integer,
  Pointer,
}

impl<'a> From<Value<'a>> for BaseType {
  fn from(value: Value<'a>) -> Self {
    match value {
      Value::Boolean(_)
      | Value::I8(_)
      | Value::I16(_)
      | Value::I32(_)
      | Value::I64(_)
      | Value::I128(_) => Self::Integer,
      Value::F16(_) | Value::F32(_) | Value::F64(_) | Value::F128(_) => Self::Float,
      Value::Pointer(..) => Self::Pointer,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
  Boolean,
  I8,
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

impl Type {
  pub fn to_base_type_enum<'a>(&self, context: &'a Context) -> BasicTypeEnum<'a> {
    match self {
        Type::Boolean => BasicTypeEnum::IntType(context.bool_type()),
        Type::I8 => BasicTypeEnum::IntType(context.i8_type()),
        Type::I16 => BasicTypeEnum::IntType(context.i16_type()),
        Type::I32 => BasicTypeEnum::IntType(context.i32_type()),
        Type::I64 => BasicTypeEnum::IntType(context.i64_type()),
        Type::I128 => BasicTypeEnum::IntType(context.i128_type()),
        Type::F16 => BasicTypeEnum::FloatType(context.f16_type()),
        Type::F32 => BasicTypeEnum::FloatType(context.f32_type()),
        Type::F64 => BasicTypeEnum::FloatType(context.f64_type()),
        Type::F128 => BasicTypeEnum::FloatType(context.f128_type()),
        Type::Pointer => panic!("Not supported now"),
    }
  }
}

impl<'a> From<&Value<'a>> for Type {
  fn from(value: &Value<'a>) -> Self {
    match value {
      Value::Boolean(_) => Self::Boolean,
      Value::I8(_) => Self::I8,
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
      Value::Boolean(_) => Self::Boolean,
      Value::I8(_) => Self::I8,
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
      "boolean" => Self::Boolean,
      "int8" => Self::I8,
      "int16" => Self::I16,
      "int32" => Self::I32,
      "int64" => Self::I64,
      "int128" => Self::I128,
      "float16" => Self::F16,
      "float32" => Self::F32,
      "float64" => Self::F64,
      "float128" => Self::F128,
      _ => unreachable!(),
    }
  }
}

impl ToString for Type {
  fn to_string(&self) -> String {
    match self {
      Type::Boolean => "boolean",
      Type::I8 => "int8",
      Type::I16 => "int16",
      Type::I32 => "int32",
      Type::I64 => "int64",
      Type::I128 => "int128",
      Type::F16 => "float16",
      Type::F32 => "float32",
      Type::F64 => "float64",
      Type::F128 => "float128",
      Type::Pointer => "*unknown",
    }
    .to_string()
  }
}
