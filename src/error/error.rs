use crate::model::InstructionFormat;
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum RISCVError {
    #[error("Invalid opcode")]
    InvalidOpcode,

    #[error("Register must be a number between 0 and 31")]
    InvalidRegister,

    #[error("Unrecognized instruction format")]
    UnrecognizedInstructionFormat,

    #[error("Provided instruction format ({0}) is invalid for the instruction")]
    UnexpectedFormat(InstructionFormat),

    #[error("Funct{0} value is too big")]
    InvalidFunctValue(u8),

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

