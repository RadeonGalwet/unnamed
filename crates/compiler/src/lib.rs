#[macro_use]
pub mod macros;
pub mod r#type;
pub mod value;

use std::collections::HashMap;

use ast::{
  Argument, Expression, Node, Operator, Statement, TopLevel, TopLevelItem, Type, UnaryOperator,
};
use inkwell::{
  basic_block::BasicBlock,
  builder::Builder,
  context::Context,
  module::Module,
  passes::PassManager,
  types::{BasicType, BasicTypeEnum, StringRadix},
  values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
};
use value::Value;

use crate::r#type::Type as RuntimeType;

pub struct Compiler<'a> {
  context: &'a Context,
  builder: Builder<'a>,
  module: Module<'a>,
  fpm: PassManager<FunctionValue<'a>>,
  named_values: HashMap<&'a str, Value<'a>>,
  variables: HashMap<&'a str, Value<'a>>,
}

impl<'a> Compiler<'a> {
  pub fn new(
    context: &'a Context,
    builder: Builder<'a>,
    module: Module<'a>,
    fpm: PassManager<FunctionValue<'a>>,
  ) -> Self {
    Self {
      context,
      builder,
      module,
      fpm,
      variables: HashMap::new(),
      named_values: HashMap::new(),
    }
  }
  pub fn start_alloca(
    &mut self,
    block: BasicBlock,
    alloca_type: BasicTypeEnum<'a>,
    name: &str,
    fn_value: FunctionValue<'a>,
  ) -> PointerValue<'a> {
    let builder = self.context.create_builder();
    let entry = fn_value.get_first_basic_block().unwrap();

    match entry.get_first_instruction() {
      Some(first_instr) => builder.position_before(&first_instr),
      None => builder.position_at_end(entry),
    }
    builder.position_at_end(block);
    builder.build_alloca(alloca_type, name)
  }
  pub fn compile(&mut self, top_level: TopLevel<'a>) -> Result<(), String> {
    self.compile_top_level(top_level)?;
    Ok(())
  }
  pub fn compile_top_level(&mut self, top_level: TopLevel<'a>) -> Result<(), String> {
    match top_level {
      TopLevel::Items(items) => {
        for item in items {
          self.compile_top_level_item(item)?;
        }
      }
    };
    Ok(())
  }
  pub fn compile_top_level_item(&mut self, item: TopLevelItem<'a>) -> Result<(), String> {
    match item {
      TopLevelItem::Function {
        name,
        arguments,
        body,
        return_type,
      } => self.compile_function(name, arguments, *body, return_type),
    }
  }
  pub fn compile_function(
    &mut self,
    name: &'a str,
    arguments: Vec<Argument<'a>>,
    body: Node<'a>,
    return_type: Type<'a>,
  ) -> Result<(), String> {
    let mut transformed_arguments = vec![];
    for argument in &arguments {
      let id = argument.argument_type.name;
      transformed_arguments.push(self.patch_type(id)?)
    }
    let return_type = self.patch_type(return_type.name)?;
    let fn_type = return_type.fn_type(transformed_arguments.as_slice(), false);
    let fn_value = self.module.add_function(name, fn_type, None);
    let block = self.context.append_basic_block(fn_value, name);
    self.builder.position_at_end(block);
    for (index, value) in fn_value.get_param_iter().enumerate() {
      let argument = &arguments[index];
      value.set_name(argument.name);
      let argument_type = transformed_arguments[index];
      let type_name = argument.argument_type.name;
      let pointer = self.start_alloca(
        block,
        argument_type,
        format!("{}_ptr", name).as_str(),
        fn_value,
      );
      self.builder.build_store(pointer, value);
      self
        .variables
        .insert(argument.name, Value::Pointer(pointer, RuntimeType::from(type_name)));
    }
    match body {
      Node::Expression(expression) => {
        let value = self.compile_expression(expression)?;
        self
          .builder
          .build_return(Some(&BasicValueEnum::from(&value)));
      }
      Node::Block(nodes) => {
        for node in nodes {
          self.compile_node(node)?;
        }
      }
      _ => unreachable!(),
    };
    if fn_value.verify(true) {
      self.fpm.run_on(&fn_value);
    } else {
      return Err("Invalid function generated".to_string());
    }
    Ok(())
  }

  pub fn patch_type(&self, string: &str) -> Result<BasicTypeEnum<'a>, String> {
    match string {
      "i16" => Ok(BasicTypeEnum::IntType(self.context.i16_type())),
      "i32" => Ok(BasicTypeEnum::IntType(self.context.i32_type())),
      "i64" => Ok(BasicTypeEnum::IntType(self.context.i64_type())),
      "i128" => Ok(BasicTypeEnum::IntType(self.context.i128_type())),
      "f16" => Ok(BasicTypeEnum::FloatType(self.context.f16_type())),
      "f32" => Ok(BasicTypeEnum::FloatType(self.context.f32_type())),
      "f64" => Ok(BasicTypeEnum::FloatType(self.context.f64_type())),
      "f128" => Ok(BasicTypeEnum::FloatType(self.context.f64_type())),
      _ => Err("User defined types not supported now".into()),
    }
  }

  pub fn compile_node(&mut self, node: Node<'a>) -> Result<Option<Value<'a>>, String> {
    match node {
      Node::Identifier(id) => {
        let mut value = if let Some(value) = self.variables.get(id) {
          *value
        } else {
          let value = self
            .named_values
            .get(id)
            .ok_or(format!("Unknown variable {}", id))?;
          *value
        };
        if let Some((ptr, ptr_type)) = value.as_pointer() {
          value = load_ptr!(ptr_type, ptr, self);
        }
        Ok(Some(value))
      }
      Node::Integer(integer) => Ok(Some(Value::I32(
        self
          .context
          .i32_type()
          .const_int_from_string(integer, StringRadix::Decimal)
          .ok_or("Invalid integer")?,
      ))),
      Node::Float(float) => Ok(Some(Value::F64(
        self.context.f64_type().const_float_from_string(float),
      ))),
      Node::Expression(expression) => Ok(Some(self.compile_expression(expression)?)),
      Node::Statement(statement) => {
        self.compile_statement(statement)?;
        Ok(None)
      }
      Node::Block(_) => unreachable!(),
    }
  }
  pub fn compile_statement(&mut self, statement: Statement<'a>) -> Result<(), String> {
    match statement {
      Statement::Return(value) => {
        if let Some(value) = value {
          let value = self.compile_node(*value)?.unwrap();
          self
            .builder
            .build_return(Some(&BasicValueEnum::from(value)))
        } else {
          self.builder.build_return(None)
        }
      }
    };
    Ok(())
  }
  pub fn compile_expression(&mut self, expression: Expression<'a>) -> Result<Value<'a>, String> {
    match expression {
      Expression::Binary { operator, lhs, rhs } => {
        let lhs = self.compile_node(*lhs)?.unwrap();
        let rhs = self.compile_node(*rhs)?.unwrap();
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
        let argument = self.compile_node(*argument)?.unwrap();
        match argument {
          Value::I16(int) => Ok(prefix!(self, operator, int, build_int_neg, I16)),
          Value::I32(int) => Ok(prefix!(self, operator, int, build_int_neg, I32)),
          Value::I64(int) => Ok(prefix!(self, operator, int, build_int_neg, I64)),
          Value::I128(int) => Ok(prefix!(self, operator, int, build_int_neg, I128)),
          Value::F16(float) => Ok(prefix!(self, operator, float, build_float_neg, F16)),
          Value::F32(float) => Ok(prefix!(self, operator, float, build_float_neg, F32)),
          Value::F64(float) => Ok(prefix!(self, operator, float, build_float_neg, F64)),
          Value::F128(float) => Ok(prefix!(self, operator, float, build_float_neg, F128)),
          _ => Err("Value can't be used in unary expressions".to_string()),
        }
      }
    }
  }
  pub fn module(&self) -> &Module<'a> {
    &self.module
  }
}

#[cfg(test)]
mod tests {
  use inkwell::{context::Context, passes::PassManager};
  use parser::Parser;

  use crate::{Compiler};

  fn check(code: &str, bytecode: &str) {
    let top_level = Parser::new(code).parse().unwrap();
    let context = Context::create();
    let module = context.create_module("tests");
    let build = context.create_builder();
    let fpm = PassManager::create(&module);
    fpm.initialize();
    let mut compiler = Compiler::new(&context, build, module, fpm);
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
      r#"
      function main() -> i32 {
        return -2;
      }
      "#,
      r#"
define i32 @main() {
main:
  ret i32 -2
}
"#,
    )
  }
  #[test]
  fn can_compile_arguments_store_and_load() {
    check(
      r#"
      function sum(a: i32) -> i32 {
        return a;
      }
      "#,
      r#"
define i32 @sum(i32 %a) {
sum:
  %sum_ptr = alloca i32, align 4
  store i32 %a, i32* %sum_ptr, align 4
  %i32_load = load i32, i32* %sum_ptr, align 4
  ret i32 %i32_load
}
"#,
    )
  }
  #[test]
  fn can_compile_infix_expression() {
    check(
      r#"
      function main() -> i32 {
        return 2 + 2;
      }
      "#,
      r#"
define i32 @main() {
main:
  ret i32 4
}
"#,
    )
  }
  #[test]
  fn can_compile_float() {
    check(
      r#"
      function main() -> f64 {
        return 2.3;
      }
      "#,
      r#"
define double @main() {
main:
  ret double 2.300000e+00
}
"#,
    )
  }
  #[test]
  fn can_compile_int() {
    check(
      r#"
      function main() -> i32 {
        return 1;
      }
      "#,
      r#"
define i32 @main() {
main:
  ret i32 1
}
"#,
    )
  }
}
