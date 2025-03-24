use crate::{
    config::UNKNOWN_MNEMONIC,
    data::get_mnemonic,
    error::RISCVError,
    model::{Funct3, Immediate, InstructionFormat, Mnemonic, Opcode, RawBitsConverter, Register},
};
use std::fmt;

use super::InstructionTrait;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct IInstruction {
    opcode: Opcode,
    rs1: Register,
    rd: Register,
    funct3: Funct3,
    imm: Immediate<0, 11>,
}

impl IInstruction {
    pub fn new(
        opcode: Opcode,
        rs1: Register,
        rd: Register,
        funct3: Funct3,
        imm: Immediate<0, 11>,
    ) -> Result<Self, RISCVError> {
        let format = opcode.format();
        if format != InstructionFormat::I {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self {
            opcode,
            rs1,
            rd,
            funct3,
            imm,
        })
    }

    pub fn rs1(&self) -> Register {
        self.rs1
    }

    pub fn rd(&self) -> Register {
        self.rd
    }

    pub fn funct3(&self) -> Funct3 {
        self.funct3
    }

    pub fn imm(&self) -> Immediate<0, 11> {
        self.imm
    }
}

impl InstructionTrait for IInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &crate::model::InstructionFormat {
        &InstructionFormat::I
    }

    fn get_mnemonic(&self) -> Option<Mnemonic> {
        get_mnemonic(self.opcode, Some(self.funct3), None)
    }

    fn immediate_bits(&self) -> u32 {
        self.imm.into_raw_bits() << 20
    }
}

impl TryFrom<u32> for IInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.format();

        if format != InstructionFormat::I {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = instr >> 20;
        let imm = Immediate::<0, 11>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            funct3: Funct3::from(instr),
            rs1: Register::from_rs1_bits(instr),
            rd: Register::from_rd_bits(instr),
            imm,
        })
    }
}

impl From<IInstruction> for u32 {
    fn from(instr: IInstruction) -> u32 {
        u32::from(instr.opcode)
            | u32::from(instr.funct3)
            | instr.rs1.into_rs1_bits()
            | instr.rd.into_rd_bits()
            | instr.immediate_bits()
    }
}

impl fmt::Display for IInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}, {}",
            self.get_mnemonic().unwrap_or(UNKNOWN_MNEMONIC.into()),
            self.rd,
            self.rs1,
            self.imm
        )
    }
}
