use super::Opcode;
use super::InstructionFormat;

pub trait InstructionTrait {
    fn get_opcode(&self) -> &Opcode;

    fn get_format(&self) -> &InstructionFormat;

    fn get_mnemonic(&self) -> &str {
        "???"
    }

    fn is_compressed(&self) -> bool {
        u8::from(*self.get_opcode()) & 0b11 == 0
    }
}


