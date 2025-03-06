use std::fmt;

use super::InstructionTrait;
use crate::config::UNKNOWN_MNEMONIC;
use crate::data::get_mnemonic;
use crate::error::RISCVError;
use crate::model::{Funct3, Immediate, InstructionFormat, Opcode, RawBitsConverter, Register};

pub struct SInstruction {
    opcode: Opcode,
    rs1: Register,
    rs2: Register,
    funct3: Funct3,
    imm: Immediate<0, 11>,
}

impl InstructionTrait for SInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &InstructionFormat {
        &InstructionFormat::S
    }

    fn get_mnemonic(&self) -> Option<&str> {
        get_mnemonic(self.opcode, Some(self.funct3), None)
    }
}

impl TryFrom<u32> for SInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.get_format();

        if format != InstructionFormat::S {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = (instr >> 7) & 0b11111 | ((instr >> 25) << 5);
        let imm = Immediate::<0, 11>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            funct3: Funct3::from(instr),
            rs1: Register::into_rs1(instr),
            rs2: Register::into_rs2(instr),
            imm,
        })
    }
}

impl fmt::Display for SInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}({})",
            self.get_mnemonic().unwrap_or(UNKNOWN_MNEMONIC),
            self.rs2,
            self.imm,
            self.rs1
        )
    }
}
