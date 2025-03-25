use std::fmt;

use super::InstructionTrait;
use crate::{
    config::UNKNOWN_MNEMONIC,
    data::get_mnemonic,
    error::RISCVError,
    model::{Immediate, InstructionFormat, Mnemonic, Opcode, RawBitsConverter, Register},
    utils::bit::{copy_bit, copy_bits},
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct JInstruction {
    opcode: Opcode,
    rd: Register,
    imm: Immediate<1, 20>,
}

impl JInstruction {
    pub fn new(opcode: Opcode, rd: Register, imm: Immediate<1, 20>) -> Result<Self, RISCVError> {
        let format = opcode.format();
        if format != InstructionFormat::J {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self { opcode, rd, imm })
    }

    pub fn rd(&self) -> Register {
        self.rd
    }

    pub fn imm(&self) -> Immediate<1, 20> {
        self.imm
    }
}

impl InstructionTrait for JInstruction {
    fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn format(&self) -> &InstructionFormat {
        &InstructionFormat::J
    }

    fn mnemonic(&self) -> Option<Mnemonic> {
        get_mnemonic(self.opcode, None, None)
    }

    fn immediate_bits(&self) -> u32 {
        let imm = self.imm.into_raw_bits();
        let mut res = 0u32;
        copy_bits(&imm, 1, &mut res, 21, 10);
        copy_bit(&imm, 11, &mut res, 20);
        copy_bits(&imm, 12, &mut res, 12, 8);
        copy_bit(&imm, 20, &mut res, 31);
        res
    }
}

fn get_raw_imm(instr: &u32) -> u32 {
    let mut res = 0u32;

    copy_bits(instr, 21, &mut res, 1, 10);
    copy_bit(instr, 20, &mut res, 11);
    copy_bits(instr, 12, &mut res, 12, 8);
    copy_bit(instr, 31, &mut res, 20);

    res
}

impl TryFrom<u32> for JInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.format();

        if format != InstructionFormat::J {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = get_raw_imm(&instr);
        let imm = Immediate::<1, 20>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            rd: Register::from_rd_bits(instr),
            imm,
        })
    }
}

impl From<JInstruction> for u32 {
    fn from(instr: JInstruction) -> u32 {
        u32::from(instr.opcode) | instr.rd.into_rd_bits() | instr.immediate_bits()
    }
}

impl fmt::Display for JInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}",
            self.mnemonic().unwrap_or(UNKNOWN_MNEMONIC.into()),
            self.rd,
            self.imm
        )
    }
}
