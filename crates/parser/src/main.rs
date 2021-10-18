use parser::Parser;

fn main() {
  let parser = Parser::new(
    r#"
  function main(a: i32) -> i32 {
    1 + 2;
    return 2;
  }
  
  "#,
  )
  .parse()
  .unwrap();
  println!("{:#?}", parser);
}
