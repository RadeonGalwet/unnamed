use enum_as_inner::EnumAsInner;
use inkwell::values::{BasicValueEnum, FloatValue, IntValue, PointerValue};

use crate::r#type::Type;

#[derive(Debug, EnumAsInner, Clone, Copy)]
pub enum Value<'a> {
  Boolean(IntValue<'a>),
  I8(IntValue<'a>),
  I16(IntValue<'a>),
  I32(IntValue<'a>),
  I64(IntValue<'a>),
  I128(IntValue<'a>),
  F16(FloatValue<'a>),
  F32(FloatValue<'a>),
  F64(FloatValue<'a>),
  F128(FloatValue<'a>),
  Pointer(PointerValue<'a>, Type),
}

impl<'a> From<&Value<'a>> for BasicValueEnum<'a> {
  fn from(value: &Value<'a>) -> Self {
    match value {
      Value::Boolean(int) | Value::I8(int) | Value::I16(int) | Value::I32(int) | Value::I64(int) | Value::I128(int) => {
        BasicValueEnum::IntValue(*int)
      }
      Value::F16(float) | Value::F32(float) | Value::F64(float) | Value::F128(float) => {
        BasicValueEnum::FloatValue(*float)
      }
      Value::Pointer(ptr, _) => BasicValueEnum::PointerValue(*ptr),
    }
  }
}

impl<'a> From<Value<'a>> for BasicValueEnum<'a> {
  fn from(value: Value<'a>) -> Self {
    match value {
      Value::Boolean(int) | Value::I8(int) | Value::I16(int) | Value::I32(int) | Value::I64(int) | Value::I128(int) => {
        BasicValueEnum::IntValue(int)
      }
      Value::F16(float) | Value::F32(float) | Value::F64(float) | Value::F128(float) => {
        BasicValueEnum::FloatValue(float)
      }
      Value::Pointer(ptr, _) => BasicValueEnum::PointerValue(ptr),
    }
  }
}
