use super::{InstructionFormat, InstructionSize, TryFromOpcodeBinary};
use crate::error::RISCVError;
use std::fmt;

pub const OPCODE_MASK: u32 = 0b1111111;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opcode(u8);

impl TryFrom<u8> for Opcode {
    type Error = RISCVError;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        if (opcode & (1 << 7)) > 0 {
            return Err(RISCVError::InvalidOpcode);
        }
        let _ = InstructionSize::try_from_opcode_binary(opcode)?;
        let _ = InstructionFormat::try_from_opcode_binary(opcode)?;
        Ok(Self(opcode))
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        opcode.0
    }
}

impl From<Opcode> for u32 {
    fn from(opcode: Opcode) -> Self {
        u8::from(opcode).into()
    }
}

impl TryFrom<u32> for Opcode {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = match u8::try_from(instr & OPCODE_MASK) {
            Ok(val) => val,
            Err(_) => unreachable!(),
        };
        Opcode::try_from(opcode)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:07b}", self.0)
    }
}

impl Opcode {
    pub fn format(&self) -> InstructionFormat {
        match InstructionFormat::try_from_opcode_binary(self.0) {
            Ok(format) => format,
            Err(_) => unreachable!(),
        }
    }

    pub fn instruction_size(&self) -> InstructionSize {
        match InstructionSize::try_from_opcode_binary(self.0) {
            Ok(size) => size,
            Err(_) => unreachable!(),
        }
    }
}
