use std::fmt;

use crate::{
    config::UNKNOWN_MNEMONIC,
    data::get_mnemonic,
    error::RISCVError,
    model::{Immediate, InstructionFormat, Opcode, RawBitsConverter, Register},
};

use super::InstructionTrait;

pub struct UInstruction {
    opcode: Opcode,
    rd: Register,
    imm: Immediate<12, 31>,
}

impl UInstruction {
    pub fn new(opcode: Opcode, rd: Register, imm: Immediate<12, 31>) -> Result<Self, RISCVError> {
        let format = opcode.format();
        if format != InstructionFormat::U {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self { opcode, rd, imm })
    }
}

impl InstructionTrait for UInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &InstructionFormat {
        &InstructionFormat::U
    }

    fn get_mnemonic(&self) -> Option<&str> {
        get_mnemonic(self.opcode, None, None)
    }

    fn immediate_bits(&self) -> u32 {
        self.imm.into_raw_bits() << 12
    }
}

impl TryFrom<u32> for UInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.format();

        if format != InstructionFormat::U {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        let imm_val = instr >> 12;
        let imm = Immediate::<12, 31>::try_from_raw_bits(imm_val)?;

        Ok(Self {
            opcode,
            rd: Register::from_rd_bits(instr),
            imm,
        })
    }
}

impl From<UInstruction> for u32 {
    fn from(instr: UInstruction) -> u32 {
        u32::from(instr.opcode) | instr.rd.into_rd_bits() | instr.immediate_bits()
    }
}

impl fmt::Display for UInstruction {
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
