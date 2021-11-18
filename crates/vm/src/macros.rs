pub macro pop($self: ident) {
  $self.stack.pop().ok_or("Stack is empty")?
}
pub macro push($self: ident, $value: expr) {
  $self.stack.push($value)
}
pub macro int($value: expr) {
  crate::value::Value::Integer($value)
}

