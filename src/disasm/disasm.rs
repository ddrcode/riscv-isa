use std::io::{self, Read};

use super::{Address, DisasmConfig, DisasmError, InstructionFormatter, InstructionRecord};
use crate::{
    instr::Instruction,
    model::{InstructionSize, TryFromOpcodeBinary},
};

pub struct Disasm {
    reader: Box<dyn Read>,
    formatter: InstructionFormatter,
    addr: Address
}

impl Disasm {
    pub fn new(reader: impl Read + 'static) -> Self {
        Self::with_config(reader, DisasmConfig::default())
    }

    pub fn with_config(reader: impl Read + 'static, config: DisasmConfig) -> Self {
        let addr = config.start_addr;
        Self {
            reader: Box::new(reader),
            formatter: InstructionFormatter::new(config),
            addr
        }
    }

    /// Attempts to read the next instruction from the reader.
    fn next_instruction(&mut self) -> Result<InstructionRecord, DisasmError> {
        let mut opcode_buf = [0u8];
        self.reader.read_exact(&mut opcode_buf)?;
        let opcode = opcode_buf[0] & 0x7f;

        let instr_size = match InstructionSize::try_from_opcode_binary(opcode) {
            Ok(InstructionSize::Size32) => 32,
            Ok(_) => {
                return Err(DisasmError::from(
                    "The disassembler doesn't currently support instructions of a size other than 32 bits"
                ))
            }
            Err(e) => return Err(DisasmError::from(e)),
        };

        let mut rest = [0u8; 3];
        self.reader
            .read_exact(&mut rest)
            .map_err(|_| DisasmError::from("Unexpected end of file"))?;

        let bytes = [opcode_buf[0], rest[0], rest[1], rest[2]];
        let instruction = Instruction::try_from_le_bytes(bytes).map_err(DisasmError::from)?;
        let record = InstructionRecord::new(instruction, self.addr);
        self.addr += instr_size;

        Ok(record)
    }

    pub fn print_next(&mut self) -> Result<(), DisasmError> {
        let record = self.next_instruction()?;
        println!("{}", self.formatter.record(&record));
        Ok(())
    }

    pub fn print_all(&mut self) -> Result<(), DisasmError> {
        let formatter = self.formatter.clone();
        self.try_for_each(|result| {
            let record = result?;
            println!("{}", formatter.record(&record));
            Ok(())
        })
    }
}

impl Iterator for Disasm {
    type Item = Result<InstructionRecord, DisasmError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_instruction() {
            Ok(record) => Some(Ok(record)),
            Err(DisasmError::IOError(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::RISCVError;
    use std::io::Cursor;

    #[test]
    fn test_disasm_next() {
        let data = vec![0x33, 0x85, 0x62, 0x00]; // add a0, t0, t1
        let cursor = Cursor::new(data);
        let mut disasm = Disasm::new(cursor);

        let record = disasm.next();
        assert!(record.is_some(), "Expected an instruction");
        let record = record.unwrap();
        match record {
            Ok(record) => {
                assert_eq!(u32::from(record.instruction()), 0x00628533);
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
