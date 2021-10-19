use crate::r#type::Type;

#[derive(Debug, Clone)]
pub struct Function<'a> {
  pub(crate) name: &'a str,
  pub(crate) type_arguments: Vec<Type>,
  pub(crate) return_type: Type
}