use crate::config::UNKNOWN_MNEMONIC;
use crate::model::{InstructionFormat, Opcode};

pub trait InstructionTrait {
    fn get_opcode(&self) -> &Opcode;

    fn get_format(&self) -> &InstructionFormat;

    fn get_mnemonic(&self) -> Option<&str> {
        Some(UNKNOWN_MNEMONIC)
    }

    fn is_compressed(&self) -> bool {
        u8::from(*self.get_opcode()) & 0b11 != 0b11
    }

}
