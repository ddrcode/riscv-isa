use std::fmt::{self};

use super::Address;
use crate::instr::Instruction;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct InstructionRecord {
    instruction: Instruction,
    address: Address,
}

impl InstructionRecord {
    pub fn new(instruction: Instruction, address: Address) -> Self {
        Self {
            instruction,
            address,
        }
    }

    pub fn instruction(&self) -> Instruction {
        self.instruction
    }

    pub fn address(&self) -> Address {
        self.address
    }
}

impl fmt::Display for InstructionRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}: {}", self.address, self.instruction)
    }
}
