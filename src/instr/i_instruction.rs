use std::fmt;
use crate::{config::UNKNOWN_MNEMONIC, data::get_mnemonic, error::RISCVError, model::{Funct3, Immediate, InstructionFormat, Opcode, RawBitsConverter, Register}};

use super::InstructionTrait;

pub struct IInstruction {
    opcode: Opcode,
    rs1: Register,
    rd: Register,
    funct3: Funct3,
    imm: Immediate<0, 11>,
}

impl InstructionTrait for IInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &crate::model::InstructionFormat {
        &InstructionFormat::I
    }

    fn get_mnemonic(&self) -> Option<&str> {
        get_mnemonic(self.opcode, Some(self.funct3), None)
    }
}

impl TryFrom<u32> for IInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.get_format();

        if format != InstructionFormat::I {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = instr >> 20;
        let imm = Immediate::<0, 11>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            funct3: Funct3::from(instr),
            rs1: Register::into_rs1(instr),
            rd: Register::into_rd(instr),
            imm,
        })
    }
}

impl fmt::Display for IInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}, {}",
            self.get_mnemonic().unwrap_or(UNKNOWN_MNEMONIC),
            self.rd,
            self.rs1,
            self.imm
        )
    }
}
