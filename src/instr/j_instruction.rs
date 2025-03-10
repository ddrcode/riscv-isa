use std::fmt;

use super::InstructionTrait;
use crate::{
    config::UNKNOWN_MNEMONIC,
    data::get_mnemonic,
    error::RISCVError,
    model::{Immediate, InstructionFormat, Opcode, RawBitsConverter, Register},
    utils::bit::{copy_bit, copy_bits},
};

pub struct JInstruction {
    opcode: Opcode,
    rd: Register,
    imm: Immediate<1, 20>,
}

impl InstructionTrait for JInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &InstructionFormat {
        &InstructionFormat::J
    }

    fn get_mnemonic(&self) -> Option<&str> {
        get_mnemonic(self.opcode, None, None)
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
        let format = opcode.get_format();

        if format != InstructionFormat::J {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = get_raw_imm(&instr);
        let imm = Immediate::<1, 20>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            rd: Register::into_rd(instr),
            imm,
        })
    }
}

impl fmt::Display for JInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}",
            self.get_mnemonic().unwrap_or(UNKNOWN_MNEMONIC),
            self.rd,
            self.imm
        )
    }
}
