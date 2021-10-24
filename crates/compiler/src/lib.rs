#[macro_use]
pub mod macros;
pub mod env;
pub mod function_signature;
pub mod state;
pub mod r#type;
pub mod value;
pub mod variable;

use std::{cell::RefCell, cmp::Ordering, collections::HashMap, rc::Rc};

use ast::{Argument, Expression, Node, Operator, Statement, TopLevel, Type, UnaryOperator};
use env::Environment;
use function_signature::FunctionSignature;
use inkwell::{
  basic_block::BasicBlock,
  builder::Builder,
  context::Context,
  module::Module,
  passes::PassManager,
  types::{BasicType, BasicTypeEnum, StringRadix},
  values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
  FloatPredicate, IntPredicate,
};
use state::State;
use value::Value;
use variable::Variable;

use crate::r#type::Type as RuntimeType;

pub struct Compiler<'a> {
  context: &'a Context,
  builder: Builder<'a>,
  module: Module<'a>,
  fpm: PassManager<FunctionValue<'a>>,
  named_values: HashMap<&'a str, Value<'a>>,
  environment: Rc<RefCell<Environment<'a, Variable<'a>>>>,
  function: Option<FunctionValue<'a>>,
  block: Option<BasicBlock<'a>>,
  functions: HashMap<&'a str, FunctionSignature<'a>>,
  state: State,
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
      state: State::default(),
      environment: Rc::new(RefCell::new(Environment::new(None))),
      named_values: HashMap::new(),
      functions: HashMap::new(),
      function: None,
      block: None,
    }
  }
  pub fn update_state(&mut self, state: State) {
    self.state = state;
  }
  pub fn reset_state(&mut self) {
    self.state = State::None;
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
    let mut signatures = vec![];
    for function in top_level.functions {
      let signature =
        self.compile_function_signature(function.name, function.arguments, function.return_type)?;
      let value = self.load_signature(&signature)?;
      signatures.push((function.name, value, function.body, signature));
    }
    for signature in signatures {
      self.compile_function(signature.0, signature.1, *(signature.2), signature.3)?;
    }
    Ok(())
  }
  pub fn compile_function_signature(
    &mut self,
    name: &'a str,
    ast_arguments: Vec<Argument<'a>>,
    return_type: Type,
  ) -> Result<FunctionSignature<'a>, String> {
    let return_type = RuntimeType::from(return_type.name);
    let mut arguments = vec![];
    for argument in ast_arguments {
      arguments.push((
        RuntimeType::from(argument.argument_type.name),
        argument.name,
      ))
    }
    let signature = FunctionSignature {
      name,
      arguments,
      return_type,
    };
    self.functions.insert(name, signature.clone()); // I don't have any ideas how to remove clone here
    Ok(signature)
  }
  pub fn load_signature(
    &mut self,
    signature: &FunctionSignature<'a>,
  ) -> Result<FunctionValue<'a>, String> {
    let mut patched_arguments = vec![];
    for argument in &signature.arguments {
      patched_arguments.push(self.patch_type(argument.0.to_string().as_str())?)
    }
    let return_type = self.patch_type(signature.return_type.to_string().as_str())?;
    let fn_type = return_type.fn_type(patched_arguments.as_slice(), false);
    Ok(self.module.add_function(signature.name, fn_type, None))
  }
  pub fn compile_function(
    &mut self,
    name: &'a str,
    fn_value: FunctionValue<'a>,
    body: Node<'a>,
    signature: FunctionSignature<'a>,
  ) -> Result<(), String> {
    let block = self.context.append_basic_block(fn_value, name);
    self.builder.position_at_end(block);
    for (index, value) in fn_value.get_param_iter().enumerate() {
      let compile_time_argument = signature.arguments[index];
      let pointer = self.start_alloca(
        block,
        value.get_type(),
        format!("load_{}_ptr", index).as_str(),
        fn_value,
      );
      self.builder.build_store(pointer, value);
      value.set_name(compile_time_argument.1);
      self.environment.borrow_mut().set(
        compile_time_argument.1,
        Variable::new(true, Value::Pointer(pointer, compile_time_argument.0)),
      );
    }
    self.function = Some(fn_value);
    self.block = Some(block);
    self.functions.insert(name, signature);
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
    self.environment.borrow_mut().clear();
    Ok(())
  }

  pub fn patch_type(&self, string: &str) -> Result<BasicTypeEnum<'a>, String> {
    match string {
      "boolean" => Ok(BasicTypeEnum::IntType(self.context.bool_type())),
      "i8" => Ok(BasicTypeEnum::IntType(self.context.i8_type())),
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

  pub fn compile_node(&mut self, node: Node<'a>) -> Result<Option<Variable<'a>>, String> {
    match node {
      Node::Identifier(id) => {
        let mut value = if let Ok(value) = self.environment.borrow_mut().get(id) {
          *value
        } else {
          let value = self
            .named_values
            .get(id)
            .ok_or(format!("Unknown variable {}", id))?;
          Variable::build_const(*value)
        };
        if let Some((ptr, ptr_type)) = value.value.as_pointer() {
          value = load_ptr!(ptr_type, ptr, self);
        }
        Ok(Some(value))
      }
      Node::Integer(integer) => Ok(Some(Variable::build_const(Value::I32(
        self
          .context
          .i32_type()
          .const_int_from_string(integer, StringRadix::Decimal)
          .ok_or("Invalid integer")?,
      )))),
      Node::Float(float) => Ok(Some(Variable::build_const(Value::F64(
        self.context.f64_type().const_float_from_string(float),
      )))),
      Node::Expression(expression) => Ok(Some(Variable::build_const(
        self.compile_expression(expression)?,
      ))),
      Node::Statement(statement) => {
        self.compile_statement(statement)?;
        Ok(None)
      }
      Node::Block(body) => {
        let old_env = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&old_env)))));
        for node in body {
          self.compile_node(node)?;
        }
        self.environment = old_env;
        Ok(None)
      }
      Node::Boolean(boolean) => Ok(Some(Variable::build_const(Value::Boolean(
        self.context.bool_type().const_int(boolean as u64, false),
      )))),
    }
  }
  pub fn compile_statement(&mut self, statement: Statement<'a>) -> Result<(), String> {
    match statement {
      Statement::Return(value) => {
        self.update_state(State::Return);
        if let Some(value) = value {
          let value = self.compile_node(*value)?.unwrap();
          self
            .builder
            .build_return(Some(&BasicValueEnum::from(value.value)))
        } else {
          self.builder.build_return(None)
        };
      }
      Statement::Conditional {
        then_branch,
        expression,
        else_branch,
      } => {
        let comparison = *(self
          .compile_node(*expression)?
          .ok_or("Not expression")?
          .value
          .as_boolean()
          .ok_or("Not boolean")?);
        let then_block = self
          .context
          .append_basic_block(self.function.unwrap(), "then");
        let else_block = self
          .context
          .append_basic_block(self.function.unwrap(), "else");
        let continue_block = self
          .context
          .append_basic_block(self.function.unwrap(), "continue");
        self
          .builder
          .build_conditional_branch(comparison, then_block, else_block);
        self.builder.position_at_end(then_block);

        self.compile_node(*then_branch)?;
        if !(self.state == State::Return) {
          self.builder.build_unconditional_branch(continue_block);
        }
        self.reset_state();
        self.builder.position_at_end(else_block);
        if let Some(else_branch) = else_branch {
          self.compile_node(*else_branch)?;
        }
        if !(self.state == State::Return) {
          self.builder.build_unconditional_branch(continue_block);
        }
        self.reset_state();
        self.builder.position_at_end(continue_block);
      }
      Statement::LetBinding {
        id,
        mutable,
        init,
        init_type,
      } => {
        if let Some(init) = init {
          let value = self.compile_node(*init)?.unwrap().value;
          let value_type = RuntimeType::from(value);
          if let Some(init_type) = init_type {
            let init_type = RuntimeType::from(init_type);
            if init_type != value_type {
              return Err(format!(
                "type '{}' is not assignable to type '{}'",
                init_type.to_string(),
                value_type.to_string()
              ));
            }
          }
          let pointer = self.builder.build_alloca(
            self.patch_type(&value_type.to_string())?,
            format!("{}_alloca", id).as_str(),
          ); // TODO: REFACTOR PATCH_TYPE
          self
            .builder
            .build_store(pointer, BasicValueEnum::from(value));
          self.environment.borrow_mut().set(
            id,
            Variable::new(mutable, Value::Pointer(pointer, value_type)),
          )
        } else {
          return Err("Variables without init not supported now".to_string());
        }
      }
    };
    Ok(())
  }
  pub fn compile_expression(&mut self, expression: Expression<'a>) -> Result<Value<'a>, String> {
    match expression {
      // TODO!!!: REFACTOR THIS SHIT
      Expression::Binary { operator, lhs, rhs } => {
        if operator == Operator::Assignment {
          let id = *(lhs.as_identifier().ok_or("Not id")?);
          let env_clone = Rc::clone(&self.environment);
          let mut env = env_clone.borrow_mut();
          let env_clone2 = env.clone();
          let variable = env_clone2.get(id)?;
          if !variable.mutable {
            return Err("Can't change immutable variable".to_string());
          }
          let value = self.compile_node(*rhs)?.unwrap().value;
          let pointer = variable.value.as_pointer().unwrap();
          self.builder.build_free(*(pointer.0));
          let new_pointer = self.builder.build_alloca(
            self.patch_type(pointer.1.to_string().as_str())?,
            format!("changed_{}_alloca", id).as_str(),
          );
          self
            .builder
            .build_store(new_pointer, BasicValueEnum::from(value));
          env.set(
            id,
            Variable::new(true, Value::Pointer(new_pointer, *(pointer.1))),
          );
          Ok(Value::Pointer(new_pointer, *(pointer.1)))
        } else {
          let lhs = self.compile_node(*lhs)?.unwrap().value;
          let rhs = self.compile_node(*rhs)?.unwrap().value;
          match (lhs, rhs) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => Ok(Value::Boolean(match operator {
              Operator::Equal => {
                self
                  .builder
                  .build_int_compare(IntPredicate::EQ, lhs, rhs, "i16_eq_cmp")
              }
              Operator::NotEqual => {
                self
                  .builder
                  .build_int_compare(IntPredicate::NE, lhs, rhs, "i16_ne_cmp")
              }
              Operator::Less => {
                self
                  .builder
                  .build_int_compare(IntPredicate::SLT, lhs, rhs, "i16_slt_cmp")
              }
              Operator::LessEqual => {
                self
                  .builder
                  .build_int_compare(IntPredicate::SLE, lhs, rhs, "i16_sle_cmp")
              }
              Operator::Greater => {
                self
                  .builder
                  .build_int_compare(IntPredicate::SGT, lhs, rhs, "i16_sgt_cmp")
              }
              Operator::GreaterEqual => {
                self
                  .builder
                  .build_int_compare(IntPredicate::SGE, lhs, rhs, "i16_sge_cmp")
              }
              Operator::And => self.builder.build_and(lhs, rhs, "i16_and"),
              Operator::Or => self.builder.build_and(lhs, rhs, "i16_or"),
              _ => return Err("Boolean don't supports arithmetic operations".to_string()),
            })),
            (Value::I8(lhs), Value::I8(rhs)) => Ok(infix_int!(self, I8, operator, lhs, rhs)),
            (Value::I16(lhs), Value::I16(rhs)) => Ok(infix_int!(self, I16, operator, lhs, rhs)),
            (Value::I32(lhs), Value::I32(rhs)) => Ok(infix_int!(self, I32, operator, lhs, rhs)),
            (Value::I64(lhs), Value::I64(rhs)) => Ok(infix_int!(self, I64, operator, lhs, rhs)),
            (Value::I128(lhs), Value::I128(rhs)) => Ok(infix_int!(self, I128, operator, lhs, rhs)),
            (Value::F16(lhs), Value::F16(rhs)) => Ok(infix_float!(self, F16, operator, lhs, rhs)),
            (Value::F32(lhs), Value::F32(rhs)) => Ok(infix_float!(self, F32, operator, lhs, rhs)),
            (Value::F64(lhs), Value::F64(rhs)) => Ok(infix_float!(self, F64, operator, lhs, rhs)),
            (Value::F128(lhs), Value::F128(rhs)) => {
              Ok(infix_float!(self, F128, operator, lhs, rhs))
            }
            _ => Err("Incompatible types in expression".into()),
          }
        }
      }
      Expression::Unary { operator, argument } => {
        let argument = self.compile_node(*argument)?.unwrap().value;
        match argument {
          Value::I8(int) => Ok(match operator {
            UnaryOperator::Minus => Value::I16(self.builder.build_int_neg(int, "i8_neg")),
          }),
          Value::I16(int) => Ok(match operator {
            UnaryOperator::Minus => Value::I16(self.builder.build_int_neg(int, "i16_neg")),
          }),
          Value::I32(int) => Ok(match operator {
            UnaryOperator::Minus => Value::I32(self.builder.build_int_neg(int, "i32_neg")),
          }),
          Value::I64(int) => Ok(match operator {
            UnaryOperator::Minus => Value::I64(self.builder.build_int_neg(int, "i64_neg")),
          }),
          Value::I128(int) => Ok(match operator {
            UnaryOperator::Minus => Value::I128(self.builder.build_int_neg(int, "i128_neg")),
          }),
          Value::F16(float) => Ok(match operator {
            UnaryOperator::Minus => Value::F16(self.builder.build_float_neg(float, "f16_neg")),
          }),
          Value::F32(float) => Ok(match operator {
            UnaryOperator::Minus => Value::F32(self.builder.build_float_neg(float, "f32_neg")),
          }),
          Value::F64(float) => Ok(match operator {
            UnaryOperator::Minus => Value::F64(self.builder.build_float_neg(float, "f64_neg")),
          }),
          Value::F128(float) => Ok(match operator {
            UnaryOperator::Minus => Value::F128(self.builder.build_float_neg(float, "f128_neg")),
          }),
          _ => Err("Value can't be used in unary expressions".to_string()),
        }
      }
      Expression::Call { name, arguments } => {
        let name = (*name)
          .as_identifier()
          .ok_or("Function name not identifier")?;
        let function = self
          .module
          .get_function(name)
          .ok_or(format!("Unknown function {}", name))?;

        match arguments.len().cmp(&(function.count_params() as usize)) {
          Ordering::Less => Err("Don't enough arguments".to_string()),
          Ordering::Greater => Err("Too much arguments".to_string()),
          Ordering::Equal => {
            let mut functions = self.functions.clone(); // REMOVE CLONE HERE
            let function_metadata = functions
              .get_mut(name)
              .ok_or(format!("Can't find function metadata for {}", name))?;
            let mut value_arguments = Vec::with_capacity(arguments.len());
            for argument in arguments {
              value_arguments.push(
                self
                  .compile_node(argument)?
                  .ok_or("Node compilation returned None")?,
              );
            }
            let mut compiled_arguments = vec![];
            for (index, value) in value_arguments.iter().enumerate() {
              let value = value.value;
              let value_type = RuntimeType::from(value);
              let expected_type = function_metadata.arguments[index];
              if value_type != expected_type.0 {
                return Err(format!(
                  "Expected {:?} type, found {:?}",
                  expected_type, value
                ));
              }

              compiled_arguments.push(BasicValueEnum::from(value));
            }
            let call_site = self.builder.build_call(
              function,
              compiled_arguments.as_slice(),
              format!("{}_call", name).as_str(),
            );
            let value = call_site
              .try_as_basic_value()
              .left()
              .ok_or("Void don't supported")?;
            match function_metadata.return_type {
              RuntimeType::Boolean => Ok(Value::Boolean(value.into_int_value())),
              RuntimeType::I8 => Ok(Value::I8(value.into_int_value())),
              RuntimeType::I16 => Ok(Value::I16(value.into_int_value())),
              RuntimeType::I32 => Ok(Value::I32(value.into_int_value())),
              RuntimeType::I64 => Ok(Value::I64(value.into_int_value())),
              RuntimeType::I128 => Ok(Value::I128(value.into_int_value())),
              RuntimeType::F16 => Ok(Value::F16(value.into_float_value())),
              RuntimeType::F32 => Ok(Value::F32(value.into_float_value())),
              RuntimeType::F64 => Ok(Value::F64(value.into_float_value())),
              RuntimeType::F128 => Ok(Value::F128(value.into_float_value())),
              RuntimeType::Pointer => Ok(Value::Pointer(
                value.into_pointer_value(),
                function_metadata.return_type,
              )),
            }
          }
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

  use crate::Compiler;

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
  %load_0_ptr = alloca i32, align 4
  store i32 %a, i32* %load_0_ptr, align 4
  %i32_load = load i32, i32* %load_0_ptr, align 4
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
  #[test]
  fn can_compile_boolean() {
    check(
      r#"
      function main() -> boolean {
        return true;
      }
      "#,
      r#"
define i1 @main() {
main:
  ret i1 true
}
"#,
    )
  }
  #[test]
  fn can_compile_logical_expression() {
    check(
      r#"
      function main() -> boolean {
        return 1.2 == 2.2;
      }
      "#,
      r#"
define i1 @main() {
main:
  ret i1 false
}
"#,
    )
  }
}
