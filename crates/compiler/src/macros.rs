#[macro_export]
macro_rules! infix {
  ($self: ident, $operator: ident, $lhs: ident, $rhs: ident, $add: ident, $sub: ident, $mul: ident, $div: ident, $target: ident) => {
    match $operator {
      Operator::Plus => Value::$target($self.builder.$add($lhs, $rhs, "tmp_add")),
      Operator::Minus => Value::$target($self.builder.$sub($lhs, $rhs, "tmp_sub")),
      Operator::Multiply => Value::$target($self.builder.$mul($lhs, $rhs, "tmp_mul")),
      Operator::Divide => Value::$target($self.builder.$div($lhs, $rhs, "tmp_div")),
      _ => todo!()
    }
  };
}

#[macro_export]
macro_rules! prefix {
  ($self: ident, $operator: ident, $argument: ident, $minus: ident, $target: ident) => {
    match $operator {
      UnaryOperator::Minus => Value::$target($self.builder.$minus($argument, "tmp_neg")),
    }
  };
}

macro_rules! load_ptr {
  ($type: ident, $value: ident, $self: ident) => {
    match $type {
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
