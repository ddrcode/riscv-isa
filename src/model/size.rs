use crate::error::RISCVError;
use std::fmt;

use super::TryFromOpcodeBinary;

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum InstructionSize {
    Size16,
    Size32,
    Size48,
    Size64,
}

impl TryFromOpcodeBinary for InstructionSize {
    fn try_from_opcode_binary(bits: u8) -> Result<Self, RISCVError> {
        if bits & 0b11 != 0b11 {
            return Ok(Self::Size16);
        }

        let bits = bits >> 2;

        if bits & 0b111 != 0b111 {
            Ok(Self::Size32)
        } else if bits & 0b1111 == 0b0111 {
            Ok(Self::Size48)
        } else if bits & 0b11111 == 0b01111 {
            Ok(Self::Size64)
        } else {
            Err(RISCVError::UnrecognizedInstructionSize)
        }
    }
}

impl From<&InstructionSize> for usize {
    fn from(size: &InstructionSize) -> Self {
        use InstructionSize::*;
        match size {
            Size16 => 16,
            Size32 => 32,
            Size48 => 48,
            Size64 => 64,
        }
    }
}

impl fmt::Display for InstructionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", usize::from(self))
    }
}

impl TryFrom<usize> for InstructionSize {
    type Error = RISCVError;

    fn try_from(size: usize) -> Result<Self, Self::Error> {
        use InstructionSize::*;
        match size {
            16 => Ok(Size16),
            32 => Ok(Size32),
            48 => Ok(Size48),
            64 => Ok(Size64),
            _ => Err(RISCVError::UnrecognizedInstructionSize),
        }
    }
}
