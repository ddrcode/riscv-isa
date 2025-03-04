use std::fmt;
use crate::error::*;
use crate::opcode::OPCODE_MASK;

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


impl TryFrom<u8> for InstructionFormat {
    type Error = RISCVError;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        use InstructionFormat::*;
        match opcode >> 2 {
            0b11000 => Ok(B),
            0b00100 | 0b00000 | 0b11001 | 0b11100 => Ok(I),
            0b11011 => Ok(J),
            0b01100 | 0b01011 | 0b01110 => Ok(R),
            0b01000 => Ok(S),
            _ => Err(RISCVError::UnrecognizedInstructionFormat)
        }
    }
}


impl TryFrom<u32> for InstructionFormat {
    type Error = RISCVError;

    fn try_from(bits: u32) -> Result<Self, Self::Error> {
        match u8::try_from(bits & OPCODE_MASK) {
            Ok(opcode) => InstructionFormat::try_from(opcode),
            Err(_) => unreachable!()
        }
    }

}
