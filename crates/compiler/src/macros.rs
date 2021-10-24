macro_rules! load_ptr {
  ($type: ident, $value: ident, $self: ident) => {
    match $type {
      RuntimeType::Boolean => Value::Boolean(
        $self
          .builder
          .build_load(*$value, "i1_load")
          .into_int_value(),
      ),
      RuntimeType::I8 => Value::I8(
        $self
          .builder
          .build_load(*$value, "i8_load")
          .into_int_value(),
      ),
      RuntimeType::I16 => Value::I16(
        $self
          .builder
          .build_load(*$value, "i16_load")
          .into_int_value(),
      ),
      RuntimeType::I32 => Value::I32(
        $self
          .builder
          .build_load(*$value, "i32_load")
          .into_int_value(),
      ),
      RuntimeType::I64 => Value::I64(
        $self
          .builder
          .build_load(*$value, "i64_load")
          .into_int_value(),
      ),
      RuntimeType::I128 => Value::I128(
        $self
          .builder
          .build_load(*$value, "i128_load")
          .into_int_value(),
      ),
      RuntimeType::F16 => Value::F16(
        $self
          .builder
          .build_load(*$value, "f16_load")
          .into_float_value(),
      ),
      RuntimeType::F32 => Value::F32(
        $self
          .builder
          .build_load(*$value, "f32_load")
          .into_float_value(),
      ),
      RuntimeType::F64 => Value::F64(
        $self
          .builder
          .build_load(*$value, "f64_load")
          .into_float_value(),
      ),
      RuntimeType::F128 => Value::F128(
        $self
          .builder
          .build_load(*$value, "f128_load")
          .into_float_value(),
      ),
      RuntimeType::Pointer => unreachable!(),
    }
  };
}
