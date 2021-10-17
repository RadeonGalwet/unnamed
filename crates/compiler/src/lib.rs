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
    // let void = self.context.void_type();
    // let main_fn_type = void.fn_type(&[], false);
    // let main = self.module.add_function("main", main_fn_type, None);
    // let entry = self.context.append_basic_block(main, "main");
    // self.builder.position_at_end(entry);
    // let compiled = self.compile_top_level(top_level)?;
    // let alloca = self
    //   .builder
    //   .build_alloca(BasicValueEnum::from(&compiled).get_type(), "tmp_alloca");
    // let _ = self
    //   .builder
    //   .build_store(alloca, BasicValueEnum::from(&compiled));
    // self.builder.build_return(None);
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
      _ => unreachable!(),
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
      } => self.compile_function(*name, arguments, *body, return_type),
    }
  }
  pub fn compile_function(
    &mut self,
    name: Node<'a>,
    arguments: Vec<Argument<'a>>,
    body: Node<'a>,
    return_type: Type<'a>,
  ) -> Result<(), String> {
    let old_named = self.named_values.clone();
    let name = name.as_identifier().unwrap();
    let mut transformed_arguments = vec![];
    for argument in &arguments {
      let id = match *argument.type_name.name {
        Node::Identifier(id) => id,
        _ => unreachable!(),
      };
      transformed_arguments.push(self.patch_type(id)?)
    }
    let return_type = self.patch_type(return_type.name.into_identifier().unwrap())?;
    let fn_type = return_type.fn_type(transformed_arguments.as_slice(), false);
    let fn_value = self.module.add_function(name, fn_type, None);
    let block = self.context.append_basic_block(fn_value, name);
    self.builder.position_at_end(block);
    for (index, value) in fn_value.get_param_iter().enumerate() {
      let argument = &arguments[index];
      let type_of = transformed_arguments[index];
      let name = match *argument.name {
        Node::Identifier(id) => id,
        _ => unreachable!(),
      };
      let type_name = match *argument.type_name.name {
        Node::Identifier(id) => id,
        _ => unreachable!(),
      };
      value.set_name(name);
      let pointer = self.start_alloca(block, type_of, format!("{}_ptr", name).as_str(), fn_value);
      self.builder.build_store(pointer, value);
      self
        .named_values
        .insert(name, Value::Pointer(pointer, RuntimeType::from(type_name)));
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
    self.named_values = old_named;
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
      Node::Identifier(id) => Ok(
        Some(
          self
            .named_values
            .get(id)
            .ok_or(format!("Unknown constant {}", id))?
            .clone(),
        ), // TODO: Remove clone here
      ),
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
        let mut lhs = self.compile_node(*lhs)?.unwrap();
        if let Some((ptr, ptr_type)) = lhs.as_pointer() {
          lhs = match ptr_type {
            RuntimeType::I16 => {
              Value::I16(self.builder.build_load(*ptr, "i16_load").into_int_value())
            }
            RuntimeType::I32 => {
              Value::I32(self.builder.build_load(*ptr, "i32_load").into_int_value())
            }
            RuntimeType::I64 => {
              Value::I64(self.builder.build_load(*ptr, "i64_load").into_int_value())
            }
            RuntimeType::I128 => {
              Value::I128(self.builder.build_load(*ptr, "i128_load").into_int_value())
            }
            RuntimeType::F16 => {
              Value::F16(self.builder.build_load(*ptr, "f16_load").into_float_value())
            }
            RuntimeType::F32 => {
              Value::F32(self.builder.build_load(*ptr, "f32_load").into_float_value())
            }
            RuntimeType::F64 => {
              Value::F64(self.builder.build_load(*ptr, "f64_load").into_float_value())
            }
            RuntimeType::F128 => Value::F128(
              self
                .builder
                .build_load(*ptr, "f128_load")
                .into_float_value(),
            ),
            RuntimeType::Pointer => unreachable!(),
          };
        }

        let mut rhs = self.compile_node(*rhs)?.unwrap();
        if let Some((ptr, ptr_type)) = rhs.as_pointer() {
          rhs = match ptr_type {
            RuntimeType::I16 => {
              Value::I16(self.builder.build_load(*ptr, "i16_load").into_int_value())
            }
            RuntimeType::I32 => {
              Value::I32(self.builder.build_load(*ptr, "i32_load").into_int_value())
            }
            RuntimeType::I64 => {
              Value::I64(self.builder.build_load(*ptr, "i64_load").into_int_value())
            }
            RuntimeType::I128 => {
              Value::I128(self.builder.build_load(*ptr, "i128_load").into_int_value())
            }
            RuntimeType::F16 => {
              Value::F16(self.builder.build_load(*ptr, "f16_load").into_float_value())
            }
            RuntimeType::F32 => {
              Value::F32(self.builder.build_load(*ptr, "f32_load").into_float_value())
            }
            RuntimeType::F64 => {
              Value::F64(self.builder.build_load(*ptr, "f64_load").into_float_value())
            }
            RuntimeType::F128 => Value::F128(
              self
                .builder
                .build_load(*ptr, "f128_load")
                .into_float_value(),
            ),
            RuntimeType::Pointer => unreachable!(),
          };
        }
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
  use std::f64::consts::PI;

  use inkwell::{context::Context, passes::PassManager};
  use parser::Parser;

  use crate::{value::Value, Compiler};

  fn check(code: &str, bytecode: &str) {
    let top_level = Parser::new(code).parse().unwrap();
    let context = Context::create();
    let module = context.create_module("tests");
    let build = context.create_builder();
    let fpm = PassManager::create(&module);
    fpm.initialize();
    let mut compiler = Compiler::new(&context, build, module, fpm);
    compiler
      .named_values
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
