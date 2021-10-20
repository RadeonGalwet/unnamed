use crate::r#type::Type;

#[derive(Debug, Clone)]
pub struct FunctionSignature<'a> {
  pub name: &'a str,
  pub arguments: Vec<(Type, &'a str)>,
  pub return_type: Type
}