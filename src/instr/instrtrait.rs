use crate::config::UNKNOWN_MNEMONIC;
use crate::model::{InstructionFormat, Mnemonic, Opcode};

pub trait InstructionTrait {
    fn opcode(&self) -> &Opcode;

    fn format(&self) -> &InstructionFormat;

    fn mnemonic(&self) -> Option<Mnemonic> {
        Some(UNKNOWN_MNEMONIC.into())
    }

    fn is_compressed(&self) -> bool {
        u8::from(*self.opcode()) & 0b11 != 0b11
    }

    fn immediate_bits(&self) -> u32;
}
