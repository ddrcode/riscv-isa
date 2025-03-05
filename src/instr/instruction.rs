use std::fmt;
use std::fmt::{Display, Formatter};

use super::*;
use crate::error::RISCVError;
use crate::model::{InstructionFormat, Opcode};

pub enum Instruction {
    R(RInstruction),
    I(IInstruction),
    S(SInstruction),
    B(BInstruction),
}

impl TryFrom<u32> for Instruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        use InstructionFormat::*;

        let instruction = match InstructionFormat::try_from(instr)? {
            R => Instruction::R(RInstruction::try_from(instr)?),
            I => Instruction::I(IInstruction::try_from(instr)?),
            S => Instruction::S(SInstruction::try_from(instr)?),
            B => Instruction::B(BInstruction::try_from(instr)?),
            _ => unreachable!(),
        };

        Ok(instruction)
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Instruction::*;
        match self {
            R(instr) => write!(f, "{}", instr),
            I(instr) => write!(f, "{}", instr),
            S(instr) => write!(f, "{}", instr),
            B(instr) => write!(f, "{}", instr),
        }
    }
}

macro_rules! delegate_instruction_methods {
    ($enum_name:ident, $trait_name:ident, $(fn $fn_name:ident(&self) -> $ret:ty),*) => {
        impl $trait_name for $enum_name {
            $(
                fn $fn_name(&self) -> $ret {
                    match self {
                        $enum_name::R(inner) => inner.$fn_name(),
                        $enum_name::I(inner) => inner.$fn_name(),
                        $enum_name::S(inner) => inner.$fn_name(),
                        $enum_name::B(inner) => inner.$fn_name(),
                        // $enum_name::I(inner) => inner.$fn_name(),
                    }
                }
            )*
        }
    };
}

delegate_instruction_methods!(Instruction, InstructionTrait,
    fn get_opcode(&self) -> &Opcode,
    fn get_format(&self) -> &InstructionFormat,
    fn get_mnemonic(&self) -> Option<&str>,
    fn is_compressed(&self) -> bool
);

