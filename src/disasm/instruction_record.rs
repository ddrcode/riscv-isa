use std::fmt;

use crate::instr::Instruction;
use super::{Address, DisasmConfig};


#[derive(Debug, PartialEq)]
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

    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn to_string_with_config(&self, config: &DisasmConfig) -> String {
        todo!()
    }
}

impl fmt::Display for InstructionRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string_with_config(&DisasmConfig::default()))
    }
}
