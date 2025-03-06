use crate::model::InstructionFormat;

#[derive(Debug, PartialEq)]
pub enum RISCVError {
    InvalidOpcode,
    UnrecognizedInstructionFormat,
    UnexpectedFormat(InstructionFormat),
    InvalidFunct3Value,
    InvalidFunct7Value,
    ImmediateOutOfRange(i32, i32),
    ImmediateBitsBeforeStart(u8)
}
