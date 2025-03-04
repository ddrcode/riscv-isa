use std::fmt;

use crate::opcode::Opcode;
use crate::format::InstructionFormat;
use crate::instrtrait::InstructionTrait;
use crate::error::RISCVError;
use crate::register::Register;
use crate::funct::Funct3;


pub struct SInstruction {
    opcode: Opcode,
    rs1: Register,
    rs2: Register,
    funct3: Funct3
}

impl InstructionTrait for SInstruction {
    fn get_opcode(&self) -> &Opcode {
        &self.opcode
    }

    fn get_format(&self) -> &InstructionFormat {
        &InstructionFormat::S
    }
}

impl TryFrom<u32> for SInstruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        let opcode = Opcode::try_from(instr)?;
        let format = opcode.get_format();

        if format != InstructionFormat::R {
            return Err(RISCVError::UnexpectedFormat(format));
        }

        Ok(Self {
            opcode,
            funct3: Funct3::from(instr),
            rs1: Register::into_rs1(instr),
            rs2: Register::into_rs2(instr),
        })
    }
}

impl fmt::Display for SInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "???")
    }
}
