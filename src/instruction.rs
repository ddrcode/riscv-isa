use std::fmt::{ Display, Formatter };
use std::fmt;

use crate::opcode::Opcode;
use crate::error::RISCVError;
use crate::format::InstructionFormat;
use crate::instrtrait::InstructionTrait;
use crate::rtype::RInstruction;
use crate::stype::SInstruction;

pub enum Instruction {
    R(RInstruction),
    S(SInstruction)
}

impl TryFrom<u32> for Instruction {
    type Error = RISCVError;

    fn try_from(instr: u32) -> Result<Self, Self::Error> {
        use InstructionFormat::*;

        let instruction = match InstructionFormat::try_from(instr)? {
            R => Instruction::R(RInstruction::try_from(instr)?),
            S => Instruction::S(SInstruction::try_from(instr)?),
            _ => unreachable!()
        };

        Ok(instruction)
    }
}

macro_rules! delegate_instruction_methods {
    ($enum_name:ident, $trait_name:ident, $(fn $fn_name:ident(&self) -> $ret:ty),*) => {
        impl $trait_name for $enum_name {
            $(
                fn $fn_name(&self) -> $ret {
                    match self {
                        $enum_name::R(inner) => inner.$fn_name(),
                        $enum_name::S(inner) => inner.$fn_name(),
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
    fn get_mnemonic(&self) -> &str,
    fn is_compressed(&self) -> bool
);


// delegate_instruction_methods!(Instruction, Display,
//     fn fmt(&self, f: &mut Formatter) -> Result
// );
//
