use crate::model::InstructionFormat;
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum RISCVError {
    #[error("Invalid opcode")]
    InvalidOpcode,

    #[error("Unrecognized instruction format")]
    UnrecognizedInstructionFormat,

    #[error("Provided instruction format ({0}) is invalid for the instruction")]
    UnexpectedFormat(InstructionFormat),

    #[error("Invalid Funct3 value")]
    InvalidFunct3Value,

    #[error("Invalid Funct7 value")]
    InvalidFunct7Value,

    #[error("Immediate value out of range. It must be between {0} and {1}")]
    ImmediateOutOfRange(i32, i32),

    #[error("Immediate cannot have any data before bit {0}")]
    ImmediateBitsBeforeStart(u8),

    #[error("Unrecognized instruction size")]
    UnrecognizedInstructionSize,

    #[error("Unidentified extension")]
    UnrecognizedExtension,

    #[error("Disassembler error: {0}")]
    DisasmError(String)
}

