use crate::model::{InstructionFormat, Mnemonic, Opcode};

pub trait InstructionTrait: Into<u32>  + TryFrom<u32> {
    fn opcode(&self) -> &Opcode;

    fn format(&self) -> &InstructionFormat;

    fn mnemonic(&self) -> Option<Mnemonic>;

    fn immediate_bits(&self) -> u32;
}
