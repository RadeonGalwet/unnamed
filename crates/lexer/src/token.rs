#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
  Identifier(&'a str),
  Integer(&'a str),
  Float(&'a str),
  Plus,
  Minus,
  Divide,
  Multiply
}