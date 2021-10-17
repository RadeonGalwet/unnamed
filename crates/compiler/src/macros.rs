#[macro_export]
macro_rules! infix {
  ($self: ident, $operator: ident, $lhs: ident, $rhs: ident, $add: ident, $sub: ident, $mul: ident, $div: ident, $target: ident) => {
    match $operator {
      Operator::Plus => Value::$target($self.builder.$add($lhs, $rhs, "tmp_add")),
      Operator::Minus => Value::$target($self.builder.$sub($lhs, $rhs, "tmp_sub")),
      Operator::Multiply => Value::$target($self.builder.$mul($lhs, $rhs, "tmp_mul")),
      Operator::Divide => Value::$target($self.builder.$div($lhs, $rhs, "tmp_div"))
    }
  };
}

#[macro_export]
macro_rules! prefix {
  ($self: ident, $operator: ident, $argument: ident, $minus: ident, $target: ident) => {
    match $operator {
      UnaryOperator::Minus => Value::$target($self.builder.$minus($argument, "tmp_neg"))
    }
  };
}