use crate::value::Value;

#[derive(Clone, Copy, Debug)]
pub struct Variable<'a> {
  pub(crate) mutable: bool,
  pub(crate) value: Value<'a>,
}

impl<'a> Variable<'a> {
  pub(crate) fn new(mutable: bool, value: Value<'a>) -> Self {
    Self { mutable, value }
  }
  pub(crate) fn build_const(value: Value<'a>) -> Self {
    Self {
      mutable: false,
      value,
    }
  }
}
