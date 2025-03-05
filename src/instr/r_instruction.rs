use std::fmt;

use super::InstructionTrait;
use crate::model::InstructionFormat;
use crate::model::Opcode;
use crate::model::Register;
use crate::model::{Funct3, Funct7};
use crate::config::UNKNOWN_MNEMONIC;
use crate::error::RISCVError;

use crate::data::get_mnemonic;

pub struct RInstruction {
    opcode: Opcode,
    rs1: Register,
    rs2: Register,
    rd: Register,
    funct3: Funct3,
    funct7: Funct7,
}

impl InstructionTrait for RInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &InstructionFormat {
        &InstructionFormat::R
    }

    fn get_mnemonic(&self) -> Option<&str> {
        get_mnemonic(self.opcode, Some(self.funct3), Some(self.funct7))
    }
}

impl TryFrom<u32> for RInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.get_format();

        if format != InstructionFormat::R {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self {
            opcode,
            rs1: Register::into_rs1(instr),
            rs2: Register::into_rs2(instr),
            rd: Register::into_rd(instr),
            funct3: Funct3::from(instr),
            funct7: Funct7::from(instr),
        })
    }
}

impl From<RInstruction> for u32 {
    fn from(instr: RInstruction) -> u32 {
        let mut bits: u32 = instr.opcode.into();
        bits |= u32::from(instr.funct3);
        bits |= u32::from(instr.funct7);
        bits |= u32::from(instr.rs1) << 15;
        bits |= u32::from(instr.rs2) << 20;
        bits |= u32::from(instr.rd) << 7;
        bits
    }
}

impl fmt::Display for RInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}, {}",
            self.get_mnemonic().unwrap_or(UNKNOWN_MNEMONIC),
            self.rd,
            self.rs1,
            self.rs2
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_instr(bits: u32) {
        let instr = RInstruction::try_from(bits).unwrap();
        assert_eq!(bits, u32::from(instr))
    }

    #[test]
    fn test_two_way_conversion() {
        assert_instr(0b00000000_10110101_00000101_00111011);
        assert_instr(0x00028533); // add a0, t0, zero
        assert_instr(0x02b504b3); // mul s1, a0, a1
        assert_instr(0x00b574b3); // and s1, a0, a1
    }
}
