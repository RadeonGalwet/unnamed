#![feature(decl_macro)]
pub mod bytecode;
pub mod macros;
pub mod result_ext;
pub mod value;
pub mod value_extract;

use std::io::{Cursor};

use bytecode::read::BytecodeRead;
use value::Value;

use crate::{
  bytecode::opcode::Opcode,
  macros::{int, pop, push},
  result_ext::ResultExt,
};

pub type VMResult = Result<(), String>;

#[derive(Debug)]
pub struct VirtualMachine {
  pub program: Cursor<Vec<u8>>,
  pub stack: Vec<Value>,
}

impl VirtualMachine {
  pub fn new(program: Vec<u8>, size: usize) -> Self {
    Self {
      program: Cursor::new(program),
      stack: Vec::with_capacity(size),
    }
  }
  pub fn run(&mut self) -> VMResult {
    while (self.program.position() as usize) < self.program.get_ref().len() {
      let opcode = self.program.read_opcode().map_err(|err| err.to_string())?;
      match opcode {
        Opcode::Halt => break,
        Opcode::PushInt => self
          .stack
          .push(Value::Integer(self.program.read_integer().vm()?)),
        Opcode::Pop => {
          pop!(self);
        }
        Opcode::AddInt => {
          let lhs = pop!(self).int()?;
          let rhs = pop!(self).int()?;
          push!(self, int!(lhs + rhs))
        },
        Opcode::SubInt => {
          let lhs = pop!(self).int()?;
          let rhs = pop!(self).int()?;
          push!(self, int!(lhs - rhs))
        },
        Opcode::MulInt => {
          let lhs = pop!(self).int()?;
          let rhs = pop!(self).int()?;
          push!(self, int!(lhs * rhs))
        },
        Opcode::DivInt => {
          let lhs = pop!(self).int()?;
          let rhs = pop!(self).int()?;
          push!(self, int!(lhs / rhs))
        },
      }
    }

    Ok(())
  }
}
