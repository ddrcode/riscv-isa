use std::fmt;

use super::*;
use crate::error::RISCVError;
use crate::model::{Funct3, Funct7, Immediate, InstructionFormat, Opcode, Register};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    R(RInstruction),
    I(IInstruction),
    S(SInstruction),
    B(BInstruction),
    U(UInstruction),
    J(JInstruction),
}

impl Instruction {
    pub fn funct3(&self) -> Option<Funct3> {
        use Instruction::*;
        match self {
            R(instr) => Some(instr.funct3()),
            I(instr) => Some(instr.funct3()),
            S(instr) => Some(instr.funct3()),
            B(instr) => Some(instr.funct3()),
            _ => None,
        }
    }

    pub fn funct7(&self) -> Option<Funct7> {
        use Instruction::*;
        match self {
            R(instr) => Some(instr.funct7()),
            _ => None,
        }
    }

    pub fn rs1(&self) -> Option<Register> {
        use Instruction::*;
        match self {
            R(instr) => Some(instr.rs1()),
            I(instr) => Some(instr.rs1()),
            S(instr) => Some(instr.rs1()),
            B(instr) => Some(instr.rs1()),
            _ => None,
        }
    }

    pub fn rs2(&self) -> Option<Register> {
        use Instruction::*;
        match self {
            R(instr) => Some(instr.rs2()),
            S(instr) => Some(instr.rs2()),
            B(instr) => Some(instr.rs2()),
            _ => None,
        }
    }

    pub fn rd(&self) -> Option<Register> {
        use Instruction::*;
        match self {
            R(instr) => Some(instr.rd()),
            I(instr) => Some(instr.rd()),
            U(instr) => Some(instr.rd()),
            J(instr) => Some(instr.rd()),
            _ => None,
        }
    }

    // pub fn imm<const START: u8, const END: u8>(&self) -> Option<Immediate<START, END>> {
    //     use Instruction::*;
    //     match self {
    //         S(instr) if START==0 && END==11 => Some(instr.imm()),
    //         // I(instr) => Some(instr.imm()),
    //         // U(instr) => Some(instr.imm()),
    //         // J(instr) => Some(instr.imm()),
    //         _ => None,
    //     }
    // }
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
            U => Instruction::U(UInstruction::try_from(instr)?),
            J => Instruction::J(JInstruction::try_from(instr)?),
        };

        Ok(instruction)
    }
}

impl From<Instruction> for u32 {
    fn from(instr: Instruction) -> Self {
        use Instruction::*;
        match instr {
            R(instr) => u32::from(instr),
            I(instr) => u32::from(instr),
            S(instr) => u32::from(instr),
            B(instr) => u32::from(instr),
            U(instr) => u32::from(instr),
            J(instr) => u32::from(instr),
        }
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
            U(instr) => write!(f, "{}", instr),
            J(instr) => write!(f, "{}", instr),
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
                        $enum_name::U(inner) => inner.$fn_name(),
                        $enum_name::J(inner) => inner.$fn_name(),
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
    fn is_compressed(&self) -> bool,
    fn immediate_bits(&self) -> u32
);

