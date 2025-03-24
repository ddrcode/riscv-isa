use std::io;

use crate::error::RISCVError;

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
