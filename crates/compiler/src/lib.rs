#[macro_use]
pub mod macros;
pub mod r#type;
pub mod value;

use std::collections::HashMap;

use ast::{Expression, Node, Operator, TopLevel, UnaryOperator};
use inkwell::{
  builder::Builder, context::Context, module::Module, types::StringRadix, values::BasicValueEnum,
};
use value::Value;

pub struct Compiler<'a> {
  context: &'a Context,
  builder: Builder<'a>,
  module: Module<'a>,
  constants: HashMap<&'a str, Value<'a>>,
}

impl<'a> Compiler<'a> {
  pub fn new(context: &'a Context, builder: Builder<'a>, module: Module<'a>) -> Self {
    Self {
      context,
      builder,
      module,
      constants: HashMap::new(),
    }
  }
  pub fn compile(&mut self, top_level: TopLevel<'a>) -> Result<(), String> {
    let void = self.context.void_type();
    let main_fn_type = void.fn_type(&[], false);
    let main = self.module.add_function("main", main_fn_type, None);
    let entry = self.context.append_basic_block(main, "main");
    self.builder.position_at_end(entry);
    let compiled = self.compile_top_level(top_level)?;
    let alloca = self
      .builder
      .build_alloca(BasicValueEnum::from(&compiled).get_type(), "tmp_alloca");
    let _ = self
      .builder
      .build_store(alloca, BasicValueEnum::from(&compiled));
    self.builder.build_return(None);
    Ok(())
  }
  pub fn compile_top_level(&mut self, top_level: TopLevel<'a>) -> Result<Value<'a>, String> {
    match top_level {
      TopLevel::Expression(node) => self.compile_node(node),
    }
  }
  pub fn compile_node(&mut self, node: Node<'a>) -> Result<Value<'a>, String> {
    match node {
      Node::Identifier(id) => Ok(
        self
          .constants
          .get(id)
          .ok_or(format!("Unknown constant {}", id))?
          .clone(), // TODO: Remove clone here
      ),
      Node::Integer(integer) => Ok(Value::I32(
        self
          .context
          .i32_type()
          .const_int_from_string(integer, StringRadix::Decimal)
          .ok_or("Invalid integer")?,
      )),
      Node::Float(float) => Ok(Value::F64(
        self.context.f64_type().const_float_from_string(float),
      )),
      Node::Expression(expression) => match expression {
        Expression::Binary { operator, lhs, rhs } => {
          let lhs = self.compile_node(*lhs)?;
          let rhs = self.compile_node(*rhs)?;
          match (lhs, rhs) {
            (Value::I16(lhs), Value::I16(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_int_add,
              build_int_sub,
              build_int_mul,
              build_int_signed_div,
              I16
            )),
            (Value::I32(lhs), Value::I32(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_int_add,
              build_int_sub,
              build_int_mul,
              build_int_signed_div,
              I32
            )),
            (Value::I64(lhs), Value::I64(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_int_add,
              build_int_sub,
              build_int_mul,
              build_int_signed_div,
              I64
            )),
            (Value::I128(lhs), Value::I128(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_int_add,
              build_int_sub,
              build_int_mul,
              build_int_signed_div,
              I128
            )),
            (Value::F16(lhs), Value::F16(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_float_add,
              build_float_sub,
              build_float_mul,
              build_float_div,
              F16
            )),
            (Value::F32(lhs), Value::F32(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_float_add,
              build_float_sub,
              build_float_mul,
              build_float_div,
              F32
            )),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_float_add,
              build_float_sub,
              build_float_mul,
              build_float_div,
              F64
            )),
            (Value::F128(lhs), Value::F128(rhs)) => Ok(infix!(
              self,
              operator,
              lhs,
              rhs,
              build_float_add,
              build_float_sub,
              build_float_mul,
              build_float_div,
              F128
            )),
            _ => Err("Incompatible types in expression".into()),
          }
        }
        Expression::Unary { operator, argument } => {
          let argument = self.compile_node(*argument)?;
          match argument {
            Value::I16(int) => Ok(prefix!(self, operator, int, build_int_neg, I16)),
            Value::I32(int) => Ok(prefix!(self, operator, int, build_int_neg, I32)),
            Value::I64(int) => Ok(prefix!(self, operator, int, build_int_neg, I64)),
            Value::I128(int) => Ok(prefix!(self, operator, int, build_int_neg, I128)),
            Value::F16(float) => Ok(prefix!(self, operator, float, build_float_neg, F16)),
            Value::F32(float) => Ok(prefix!(self, operator, float, build_float_neg, F32)),
            Value::F64(float) => Ok(prefix!(self, operator, float, build_float_neg, F64)),
            Value::F128(float) => Ok(prefix!(self, operator, float, build_float_neg, F128)),
          }
        }
      },
    }
  }
  pub fn module(&self) -> &Module<'a> {
    &self.module
  }
}

#[cfg(test)]
mod tests {
  use std::f64::consts::PI;

  use inkwell::context::Context;
  use parser::Parser;

  use crate::{value::Value, Compiler};

  fn check(code: &str, bytecode: &str) {
    let top_level = Parser::new(code).parse().unwrap();
    let context = Context::create();
    let module = context.create_module("tests");
    let build = context.create_builder();
    let mut compiler = Compiler::new(&context, build, module);
    compiler
      .constants
      .insert("pi", Value::F64(context.f64_type().const_float(PI)));
    compiler.compile(top_level).unwrap();
    assert_eq!(
      compiler.module().print_to_string().to_string().as_str(),
      format!(
        r#"; ModuleID = 'tests'
source_filename = "tests"
{}"#,
        bytecode
      )
    );
  }
  #[test]
  fn can_compile_unary_expression() {
    check(
      "-2",
      r#"
define void @main() {
main:
  %tmp_alloca = alloca i32, align 4
  store i32 -2, i32* %tmp_alloca, align 4
  ret void
}
"#,
    )
  }
  #[test]
  fn can_compile_infix_expression() {
    check(
      "2 + 2",
      r#"
define void @main() {
main:
  %tmp_alloca = alloca i32, align 4
  store i32 4, i32* %tmp_alloca, align 4
  ret void
}
"#,
    )
  }
  #[test]
  fn can_compile_constant() {
    check(
      "pi",
      r#"
define void @main() {
main:
  %tmp_alloca = alloca double, align 8
  store double 0x400921FB54442D18, double* %tmp_alloca, align 8
  ret void
}
"#,
    )
  }
  #[test]
  fn can_compile_int() {
    check(
      "1",
      r#"
define void @main() {
main:
  %tmp_alloca = alloca i32, align 4
  store i32 1, i32* %tmp_alloca, align 4
  ret void
}
"#,
    )
  }
  #[test]
  fn can_compile_float() {
    check(
      "1.2",
      r#"
define void @main() {
main:
  %tmp_alloca = alloca double, align 8
  store double 1.200000e+00, double* %tmp_alloca, align 8
  ret void
}
"#,
    )
  }
}
