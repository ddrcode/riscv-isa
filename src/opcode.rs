use std::fmt;
use crate::error::RISCVError;
use crate::format::InstructionFormat;

pub const OPCODE_MASK: u32 = 0b1111111;

#[derive(Debug, Clone, Copy)]
pub struct Opcode(u8);

impl TryFrom<u8> for Opcode {
    type Error = RISCVError;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        if (opcode & (1<<7)) > 0 {
            return Err(RISCVError::InvalidOpcode);
        }
        let _ = InstructionFormat::try_from(opcode)?;
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
            Err(_) => unreachable!()
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
    pub fn get_format(&self) -> InstructionFormat {
        match InstructionFormat::try_from(self.0) {
            Ok(format) => format,
            Err(_) => unreachable!()
        }
    }
}

