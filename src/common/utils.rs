pub fn get_utf8_slice(s: &str, begin: usize, end: usize) -> Option<&str> {
  if end < begin {
    return None;
  }

  s.char_indices().nth(begin).and_then(|(start_pos, _)| {
    if end >= s.len() {
      return Some(&s[start_pos..]);
    }

    s[start_pos..]
      .char_indices()
      .nth(end - begin)
      .map(|(end_pos, _)| &s[start_pos..start_pos + end_pos])
  })
}
#[macro_export]
macro_rules! min {
  ($x: expr) => ($x);
  ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}
#[macro_export]
macro_rules! max {
  ($x: expr) => ($x);
  ($x: expr, $($z: expr),+) => (::std::cmp::max($x, max!($($z),*)));
}
