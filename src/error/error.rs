use crate::model::InstructionFormat;

#[derive(Debug)]
pub enum RISCVError {
    InvalidOpcode,
    UnrecognizedInstructionFormat,
    UnexpectedFormat(InstructionFormat),
    InvalidFunct3Value,
    InvalidFunct7Value,
    ImmediateTooBig(u8)
}
