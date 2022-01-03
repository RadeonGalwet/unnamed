

#[derive(Clone, Copy, Debug)]
pub struct Span<T: Clone + Copy> {
  pub start: T,
  pub end: T,
}

impl<T: Clone + Copy> Span<T> {
  pub fn new(start: T, end: T) -> Self {
    Self { start, end }
  }

}
