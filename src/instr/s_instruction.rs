use std::fmt;

use super::InstructionTrait;
use crate::config::UNKNOWN_MNEMONIC;
use crate::data::get_mnemonic;
use crate::error::RISCVError;
use crate::model::{Funct3, Immediate, InstructionFormat, Opcode, RawBitsConverter, Register};

#[derive(Debug, PartialEq)]
pub struct SInstruction {
    opcode: Opcode,
    rs1: Register,
    rs2: Register,
    funct3: Funct3,
    imm: Immediate<0, 11>,
}

impl SInstruction {
    pub fn new(
        opcode: Opcode,
        rs1: Register,
        rs2: Register,
        funct3: Funct3,
        imm: Immediate<0, 11>,
    ) -> Result<Self, RISCVError> {
        let format = opcode.format();
        if format != InstructionFormat::S {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self {
            opcode,
            rs1,
            rs2,
            funct3,
            imm,
        })
    }

    pub fn rs1(&self) -> Register {
        self.rs1
    }

    pub fn rs2(&self) -> Register {
        self.rs2
    }

    pub fn funct3(&self) -> Funct3 {
        self.funct3
    }

    pub fn imm(&self) -> Immediate<0, 11> {
        self.imm
    }
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

    fn immediate_bits(&self) -> u32 {
        let bits = self.imm.into_raw_bits();
        ((bits & 0b11111) << 7) | ((bits >> 5) << 25)
    }
}

impl TryFrom<u32> for SInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.format();

        if format != InstructionFormat::S {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = (instr >> 7) & 0b11111 | ((instr >> 25) << 5);
        let imm = Immediate::<0, 11>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            funct3: Funct3::from(instr),
            rs1: Register::from_rs1_bits(instr),
            rs2: Register::from_rs2_bits(instr),
            imm,
        })
    }
}

impl From<SInstruction> for u32 {
    fn from(instr: SInstruction) -> u32 {
        u32::from(instr.opcode)
            | u32::from(instr.funct3)
            | instr.rs1.into_rs1_bits()
            | instr.rs2.into_rs2_bits()
            | instr.immediate_bits()
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
