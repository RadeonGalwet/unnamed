use parser::Parser;

fn main() {
  let parser = Parser::new(
    r#"
  function main(a: i32) -> i32 {
    1 + 2;
    return 2;
  }
  
  function sum(a: i32, b: i32) -> i32 {
    return a + b;
  }
  "#,
  )
  .parse()
  .unwrap();
  println!("{:#?}", parser);
}
