use super::r#type::Type;

#[derive(Debug)]
pub enum InferResult {
  Cast(Type),
  Success(Type)
}