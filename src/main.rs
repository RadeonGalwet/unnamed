use std::time::Instant;

use unnamed::{lexer::{Lexer}, common::source::Source};

fn main() {
  let mut lexer = Lexer::new(Source {
    code: r#"
    1.1 + 2.2 // Calculate
    /*
      This is block comment
    */
    2 + юникод * 2
    "#,
    path: "main.ul"
  });
  let instant = Instant::now();
  while let Ok(_) = lexer.next_token() {
  }
  let elapsed = instant.elapsed();
  println!("{:?}", elapsed)
  
}
