use super::source::Source;

#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
  pub start: usize,
  pub end: usize,
  pub source: Source<'a>,
}

impl<'a> Span<'a> {
  pub fn new(start: usize, end: usize, source: Source<'a>) -> Self {
    Self { start, end, source }
  }
  pub fn expand(&self) -> &'a str {
    &self.source.code[self.start..self.end]
  }
}
