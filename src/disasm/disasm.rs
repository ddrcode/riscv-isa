use std::io::{self, Read};

use crate::{
    error::RISCVError,
    instr::Instruction,
    model::{InstructionSize, TryFromOpcodeBinary},
};

pub enum DisasmError {
    IOError(io::Error),
    RISCVError(RISCVError),
}

impl From<io::Error> for DisasmError {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<RISCVError> for DisasmError {
    fn from(err: RISCVError) -> Self {
        Self::RISCVError(err)
    }
}

impl From<&str> for DisasmError {
    fn from(msg: &str) -> Self {
        Self::RISCVError(RISCVError::DisasmError(msg.to_string()))
    }
}

pub struct Disasm {
    reader: dyn Read,
}

impl Iterator for Disasm {
    type Item = Result<Instruction, DisasmError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut opcode_buff: [u8; 1] = [0];
        match self.reader.read(&mut opcode_buff) {
            Ok(0) => return None,
            Ok(_) => {}
            Err(e) => return Some(Err(DisasmError::from(e))),
        };

        let opcode = opcode_buff[0] & 0x7f;
        match InstructionSize::try_from_opcode_binary(opcode) {
            Ok(InstructionSize::Size32) => {},
            Ok(_) => return Some(Err(DisasmError::from(
               "The disassembler doesn't currently support instructions of different size than 32 bits"))),
            Err(e) => return Some(Err(DisasmError::from(e)))
        };

        let mut buff: [u8; 3] = [0; 3];
        match self.reader.read(&mut buff) {
            Ok(3) => {}
            Ok(_) => return Some(Err(DisasmError::from("Unexpected end of file"))),
            Err(e) => return Some(Err(DisasmError::from(e))),
        };

        let bytes = std::array::from_fn(|i| if i < 1 { opcode_buff[i] } else { buff[i - 1] });

        match Instruction::try_from(u32::from_le_bytes(bytes)) {
            Ok(instr) => Some(Ok(instr)),
            Err(e) => Some(Err(DisasmError::from(e)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disasm() {}
}
