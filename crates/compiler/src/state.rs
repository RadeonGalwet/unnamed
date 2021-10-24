impl Default for State {
  fn default() -> Self {
    Self::None
  }
}
#[derive(PartialEq, Debug)]
pub enum State {
  Return,
  None,
}
