use enum_as_inner::EnumAsInner;
use inkwell::values::{BasicValueEnum, FloatValue, IntValue};

#[derive(Debug, EnumAsInner, Clone)]
pub enum Value<'a> {
  I16(IntValue<'a>),
  I32(IntValue<'a>),
  I64(IntValue<'a>),
  I128(IntValue<'a>),
  F16(FloatValue<'a>),
  F32(FloatValue<'a>),
  F64(FloatValue<'a>),
  F128(FloatValue<'a>),
}

impl<'a> From<&Value<'a>> for BasicValueEnum<'a> {
  fn from(value: &Value<'a>) -> Self {
    match value {
      Value::I16(int) | Value::I32(int) | Value::I64(int) | Value::I128(int) => {
        BasicValueEnum::IntValue(*int)
      }
      Value::F16(float) | Value::F32(float) | Value::F64(float) | Value::F128(float) => {
        BasicValueEnum::FloatValue(*float)
      }
    }
  }
}
