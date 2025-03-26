use std::fmt;

use super::InstructionTrait;
use crate::config::UNKNOWN_MNEMONIC;
use crate::error::RISCVError;
use crate::model::InstructionFormat;
use crate::model::Mnemonic;
use crate::model::Opcode;
use crate::model::Register;
use crate::model::{Funct3, Funct7};

use crate::data::get_mnemonic;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RInstruction {
    opcode: Opcode,
    rs1: Register,
    rs2: Register,
    rd: Register,
    funct3: Funct3,
    funct7: Funct7,
}

impl RInstruction {
    pub fn new(
        opcode: Opcode,
        rs1: Register,
        rs2: Register,
        rd: Register,
        funct3: Funct3,
        funct7: Funct7,
    ) -> Result<Self, RISCVError> {
        let format = opcode.format();
        if format != InstructionFormat::R {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self {
            opcode,
            rs1,
            rs2,
            rd,
            funct3,
            funct7,
        })
    }

    pub fn rs1(&self) -> Register {
        self.rs1
    }

    pub fn rs2(&self) -> Register {
        self.rs2
    }

    pub fn rd(&self) -> Register {
        self.rd
    }

    pub fn funct3(&self) -> Funct3 {
        self.funct3
    }

    pub fn funct7(&self) -> Funct7 {
        self.funct7
    }
}

impl InstructionTrait for RInstruction {
    fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn format(&self) -> &InstructionFormat {
        &InstructionFormat::R
    }

    fn mnemonic(&self) -> Option<Mnemonic> {
        get_mnemonic(self.opcode, Some(self.funct3), Some(self.funct7))
    }

    fn immediate_bits(&self) -> u32 {
        0
    }
}

impl TryFrom<u32> for RInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.format();

        if format != InstructionFormat::R {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self {
            opcode,
            rs1: Register::from_rs1_bits(instr),
            rs2: Register::from_rs2_bits(instr),
            rd: Register::from_rd_bits(instr),
            funct3: Funct3::from(instr),
            funct7: Funct7::from(instr),
        })
    }
}

impl From<RInstruction> for u32 {
    fn from(instr: RInstruction) -> Self {
        u32::from(&instr)
    }
}

impl From<&RInstruction> for u32 {
    fn from(instr: &RInstruction) -> u32 {
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
            self.mnemonic().unwrap_or(UNKNOWN_MNEMONIC.into()),
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
        assert_instr(0x00028533); // add a0, t0, zero
        assert_instr(0x02b504b3); // mul s1, a0, a1
        assert_instr(0x00b574b3); // and s1, a0, a1
    }
}
