pub trait ResultExt<T> {
  fn vm(&self) -> Result<T, String>;
}
impl<T: Clone + Copy, E: ToString> ResultExt<T> for Result<T, E> {
  fn vm(&self) -> Result<T, String> {
    match self {
      Ok(value) => Ok(*value),
      Err(err) => Err(err.to_string()),
    }
  }
}
