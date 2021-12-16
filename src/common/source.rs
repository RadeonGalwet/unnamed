#[derive(Debug, Clone, Copy)]
pub struct Source<'a> {
  pub code: &'a str,
  pub path: &'a str,
}

impl<'a> Source<'a> {
  pub fn len(&self) -> usize {
    self.code.chars().count()
  }
}