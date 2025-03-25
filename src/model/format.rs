use super::{TryFromOpcodeBinary, OPCODE_MASK};
use crate::error::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum InstructionFormat {
    B,
    I,
    J,
    R,
    S,
    U,
}

impl fmt::Display for InstructionFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-Type", self)
    }
}

impl TryFrom<u32> for InstructionFormat {
    type Error = RISCVError;

    fn try_from(bits: u32) -> Result<Self, Self::Error> {
        match u8::try_from(bits & OPCODE_MASK) {
            Ok(opcode) => InstructionFormat::try_from_opcode_binary(opcode),
            Err(_) => unreachable!("As the value is masked, it's always 7-bits, so it fits in u8"),
        }
    }
}

impl TryFromOpcodeBinary for InstructionFormat {
    fn try_from_opcode_binary(opcode: u8) -> Result<Self, RISCVError> {
        use InstructionFormat::*;
        match opcode >> 2 {
            0b11000 => Ok(B),
            0b00100 | 0b00000 | 0b11001 | 0b11100 => Ok(I),
            0b11011 => Ok(J),
            0b01100 | 0b01011 | 0b01110 => Ok(R),
            0b01000 => Ok(S),
            0b01101 => Ok(U),
            _ => Err(RISCVError::UnrecognizedInstructionFormat),
        }
    }
}
