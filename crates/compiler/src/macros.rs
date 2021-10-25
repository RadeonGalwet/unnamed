macro_rules! infix_int {
  ($self: ident, $target: ident, $operator: ident, $lhs: ident, $rhs: ident) => {
    match $operator {
      Operator::Plus => Value::$target($self.builder.build_int_add($lhs, $rhs, "add")),
      Operator::Minus => Value::$target($self.builder.build_int_sub($lhs, $rhs, "sub")),
      Operator::Multiply => Value::$target($self.builder.build_int_mul($lhs, $rhs, "mul")),
      Operator::Divide => Value::$target($self.builder.build_int_signed_div($lhs, $rhs, "div")),
      Operator::Equal => Value::Boolean($self.builder.build_int_compare(
        IntPredicate::EQ,
        $lhs,
        $rhs,
        "eq_cmp",
      )),
      Operator::NotEqual => Value::Boolean($self.builder.build_int_compare(
        IntPredicate::NE,
        $lhs,
        $rhs,
        "ne_cmp",
      )),
      Operator::Less => Value::Boolean($self.builder.build_int_compare(
        IntPredicate::SLT,
        $lhs,
        $rhs,
        "slt_cmp",
      )),
      Operator::LessEqual => Value::Boolean($self.builder.build_int_compare(
        IntPredicate::SLE,
        $lhs,
        $rhs,
        "sle_cmp",
      )),
      Operator::Greater => Value::Boolean($self.builder.build_int_compare(
        IntPredicate::SGT,
        $lhs,
        $rhs,
        "sgt_cmp",
      )),
      Operator::GreaterEqual => Value::Boolean($self.builder.build_int_compare(
        IntPredicate::SGE,
        $lhs,
        $rhs,
        "sge_cmp",
      )),
      Operator::And => Value::$target($self.builder.build_and($lhs, $rhs, "and")),
      Operator::Or => Value::$target($self.builder.build_and($lhs, $rhs, "or")),
      Operator::Assignment => unreachable!(),
    }
  };
}

macro_rules! infix_float {
  ($self: ident, $target: ident, $operator: ident, $lhs: ident, $rhs: ident) => {
    match $operator {
      Operator::Plus => Value::$target($self.builder.build_float_add($lhs, $rhs, "add")),
      Operator::Minus => Value::$target($self.builder.build_float_sub($lhs, $rhs, "sub")),
      Operator::Multiply => Value::$target($self.builder.build_float_mul($lhs, $rhs, "mul")),
      Operator::Divide => Value::$target($self.builder.build_float_div($lhs, $rhs, "div")),
      Operator::Equal => Value::Boolean($self.builder.build_float_compare(
        FloatPredicate::OEQ,
        $lhs,
        $rhs,
        "eq_cmp",
      )),
      Operator::NotEqual => Value::Boolean($self.builder.build_float_compare(
        FloatPredicate::ONE,
        $lhs,
        $rhs,
        "ne_cmp",
      )),
      Operator::Less => Value::Boolean($self.builder.build_float_compare(
        FloatPredicate::OLT,
        $lhs,
        $rhs,
        "slt_cmp",
      )),
      Operator::LessEqual => Value::Boolean($self.builder.build_float_compare(
        FloatPredicate::OLE,
        $lhs,
        $rhs,
        "sle_cmp",
      )),
      Operator::Greater => Value::Boolean($self.builder.build_float_compare(
        FloatPredicate::OGT,
        $lhs,
        $rhs,
        "sgt_cmp",
      )),
      Operator::GreaterEqual => Value::Boolean($self.builder.build_float_compare(
        FloatPredicate::OGE,
        $lhs,
        $rhs,
        "sge_cmp",
      )),
      Operator::And => return Err("Floats don't support bits operations".to_string()),
      Operator::Or => return Err("Floats don't support bits operations".to_string()),
      Operator::Assignment => unreachable!(),
    }
  };
}

macro_rules! load_ptr {
  ($type: ident, $value: ident, $self: ident) => {
    Variable::build_const(match $type {
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
    })
  };
}

macro_rules! expr_value {
  ($value: expr) => {
    Ok(Some($value))
  };
}
macro_rules! none {
  () => {
    Ok(None)
  };
}