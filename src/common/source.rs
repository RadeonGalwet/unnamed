#[derive(Clone, Copy, Debug)]
pub struct Source<'a> {
  pub code: &'a str,
  pub path: &'a str
}
