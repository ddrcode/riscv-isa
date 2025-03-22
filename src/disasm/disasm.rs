use std::io::{self, Read};

use crate::{
    error::RISCVError,
    instr::Instruction,
    model::{InstructionSize, TryFromOpcodeBinary},
};

use super::DisasmConfig;


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
    config: DisasmConfig
}

impl Disasm {

    pub fn new(reader: impl Read + 'static) -> Self {
        Self::with_config(reader, DisasmConfig::default())
    }

    pub fn with_config(reader: impl Read + 'static, config: DisasmConfig) -> Self {
        Self {
            reader: Box::new(reader),
            config
        }
    }

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
        let data = vec![0x33, 0x85, 0x62, 0x00]; // add a0, t0, t1
        let cursor = Cursor::new(data);
        let mut disasm = Disasm::new(cursor);

        let instr = disasm.next();
        assert!(instr.is_some(), "Expected an instruction");
        let instr = instr.unwrap();
        match instr {
            Ok(instr) => {
                assert_eq!(u32::from(instr), 0x00628533);
            }
            Err(e) => panic!("Expected Ok(Instruction), got error: {:?}", e),
        }

        assert!(disasm.next().is_none());
    }

    #[test]
    fn test_disasm_unexpected_eof() {
        let data = vec![0x33, 0x00];
        let cursor = Cursor::new(data);
        let mut disasm = Disasm::new(cursor);

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
