use std::io::{self, Read};

use crate::{
    error::RISCVError,
    instr::Instruction,
    model::{InstructionSize, TryFromOpcodeBinary},
};


#[derive(Debug)]
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
    reader: Box<dyn Read>,
}

impl Disasm {
    /// Attempts to read the next instruction from the reader.
    fn next_instruction(&mut self) -> Result<Instruction, DisasmError> {
        // Read the first byte (which contains the opcode).
        let mut opcode_buf = [0u8];
        // Using read_exact so that we get an error if no byte is available.
        self.reader.read_exact(&mut opcode_buf)?;
        let opcode = opcode_buf[0] & 0x7f;

        // Determine instruction size.
        match InstructionSize::try_from_opcode_binary(opcode) {
            Ok(InstructionSize::Size32) => { /* ok */ }
            Ok(_) => {
                return Err(DisasmError::from(
                    "The disassembler doesn't currently support instructions of a size other than 32 bits"
                ))
            }
            Err(e) => return Err(DisasmError::from(e)),
        };

        // Read the remaining 3 bytes.
        let mut rest = [0u8; 3];
        self.reader.read_exact(&mut rest)
            .map_err(|_| DisasmError::from("Unexpected end of file"))?;

        // Combine into a 4-byte little-endian value.
        let full_bytes = [opcode_buf[0], rest[0], rest[1], rest[2]];
        Instruction::try_from(u32::from_le_bytes(full_bytes))
            .map_err(DisasmError::from)
    }
}

impl Iterator for Disasm {
    type Item = Result<Instruction, DisasmError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Try reading the next instruction.
        // Here, if we hit an EOF, we convert that to None, ending iteration.
        match self.next_instruction() {
            Ok(instr) => Some(Ok(instr)),
            Err(DisasmError::IOError(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_disasm_next() {
        // Prepare a valid 32-bit instruction:
        // For our dummy InstructionSize::try_from_opcode_binary, opcode must be 0x33.
        // We'll use the following 4 bytes: [0x33, 0x00, 0x00, 0x00].
        let data = vec![0x33, 0x00, 0x00, 0x00];
        let cursor = Cursor::new(data);
        let mut disasm = Disasm {
            reader: Box::new(cursor),
        };

        // The first (and only) instruction should be read successfully.
        let instr = disasm.next();
        assert!(instr.is_some(), "Expected an instruction");
        let instr = instr.unwrap();
        match instr {
            Ok(instr) => {
                assert_eq!(u32::from(instr), 0x33000000);
            }
            Err(e) => panic!("Expected Ok(Instruction), got error: {:?}", e),
        }

        // Next call should return None (no more instructions).
        assert!(disasm.next().is_none());
    }

    #[test]
    fn test_disasm_unexpected_eof() {
        // Prepare data that doesn't have enough bytes for a complete instruction.
        let data = vec![0x33, 0x00]; // Only 2 bytes
        let cursor = Cursor::new(data);
        let mut disasm = Disasm {
            reader: Box::new(cursor),
        };

        // We expect an error wrapped in Some(...) rather than None.
        let result = disasm.next();
        assert!(result.is_some(), "Expected an error due to EOF");
        match result.unwrap() {
            Err(DisasmError::RISCVError(RISCVError::DisasmError(msg))) => {
                assert_eq!(msg, "Unexpected end of file");
            }
            Err(e) => panic!("Unexpected error variant: {:?}", e),
            Ok(_) => panic!("Expected an error, but got an instruction"),
        }
    }
}
