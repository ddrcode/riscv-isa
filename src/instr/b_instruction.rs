use std::fmt;

use super::InstructionTrait;
use crate::config::UNKNOWN_MNEMONIC;
use crate::data::get_mnemonic;
use crate::error::RISCVError;
use crate::model::{
    Funct3, Immediate, InstructionFormat, Mnemonic, Opcode, RawBitsConverter, Register,
};
use crate::utils::bit::{copy_bit, copy_bits};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BInstruction {
    opcode: Opcode,
    rs1: Register,
    rs2: Register,
    funct3: Funct3,
    imm: Immediate<1, 12>,
}

impl BInstruction {
    pub fn new(
        opcode: Opcode,
        rs1: Register,
        rs2: Register,
        funct3: Funct3,
        imm: Immediate<1, 12>,
    ) -> Result<Self, RISCVError> {
        let format = opcode.format();
        if format != InstructionFormat::B {
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

    pub fn imm(&self) -> Immediate<1, 12> {
        self.imm
    }
}

fn get_raw_imm(instr: &u32) -> u32 {
    let mut res = 0u32;

    copy_bits(instr, 8, &mut res, 0, 4);
    copy_bits(instr, 25, &mut res, 4, 6);
    copy_bit(instr, 7, &mut res, 10);
    copy_bit(instr, 31, &mut res, 11);

    res
}

impl InstructionTrait for BInstruction {
    fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn format(&self) -> &InstructionFormat {
        &InstructionFormat::B
    }

    fn mnemonic(&self) -> Option<Mnemonic> {
        get_mnemonic(self.opcode, Some(self.funct3), None)
    }

    fn immediate_bits(&self) -> u32 {
        let imm = &self.imm.into_raw_bits();
        let mut res = 0u32;
        copy_bits(imm, 0, &mut res, 8, 4);
        copy_bits(imm, 4, &mut res, 25, 6);
        copy_bit(imm, 10, &mut res, 7);
        copy_bit(imm, 11, &mut res, 31);
        res
    }
}

impl TryFrom<u32> for BInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.format();

        if format != InstructionFormat::B {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = get_raw_imm(&instr);
        let imm = Immediate::<1, 12>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            funct3: Funct3::from(instr),
            rs1: Register::from_rs1_bits(instr),
            rs2: Register::from_rs2_bits(instr),
            imm,
        })
    }
}

impl From<BInstruction> for u32 {
    fn from(instr: BInstruction) -> Self {
        u32::from(&instr)
    }
}

impl From<&BInstruction> for u32 {
    fn from(instr: &BInstruction) -> u32 {
        u32::from(instr.opcode)
            | u32::from(instr.funct3)
            | instr.rs1.into_rs1_bits()
            | instr.rs2.into_rs2_bits()
            | instr.immediate_bits()
    }
}

impl fmt::Display for BInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}, {}, {}",
            self.mnemonic().unwrap_or(UNKNOWN_MNEMONIC.into()),
            self.rs1,
            self.rs2,
            self.imm
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_immediate_bits() -> Result<(), RISCVError> {
        let instr = BInstruction::try_from(0x00b64463)?; // blt a2, a1, 8
        assert_eq!(0b1000 << 7, instr.immediate_bits());
        assert_eq!(8, instr.imm().into());
        Ok(())
    }
}
