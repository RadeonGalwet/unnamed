use byteorder::{LittleEndian, ReadBytesExt};

use super::opcode::Opcode;
use std::io::{Error, ErrorKind, Read, Result};

pub trait BytecodeRead {
  fn read_opcode(&mut self) -> Result<Opcode>;
  fn read_integer(&mut self) -> Result<i32>;
  fn read_byte(&mut self) -> Result<u8>;
}
impl<T> BytecodeRead for T
where
  T: Read,
{
  fn read_opcode(&mut self) -> Result<Opcode> {
    match self.read_byte()? {
      0b0 => Ok(Opcode::Halt),
      0b1 => Ok(Opcode::PushInt),
      0b10 => Ok(Opcode::AddInt),
      0b11 => Ok(Opcode::SubInt),
      0b100 => Ok(Opcode::MulInt),
      0b101 => Ok(Opcode::DivInt),
      _ => Err(Error::new(ErrorKind::InvalidData, "Unknown opcode")),
    }
  }

  fn read_integer(&mut self) -> Result<i32> {
    self.read_i32::<LittleEndian>()
  }

  fn read_byte(&mut self) -> Result<u8> {
    let mut buffer = [0; 1];
    self.read_exact(&mut buffer)?;
    Ok(buffer[0])
  }
}
