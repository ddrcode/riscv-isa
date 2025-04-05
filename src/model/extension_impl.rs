use crate::error::RISCVError;

use super::{RISCVExtension, TryFromOpcodeBinary};

impl TryFromOpcodeBinary for RISCVExtension {
    fn try_from_opcode_binary(bits: u8) -> Result<Self, crate::error::RISCVError> {
        use RISCVExtension::*;
        let x = (bits >> 2) & 0b11111;
        if (bits & 0b11 == 0b11) && (x == 0b00010 || x == 0b01010 || x == 0b10110 || x == 0b11110) {
            Ok(Custom)
        } else {
            Err(RISCVError::UnrecognizedExtension)
        }
    }
}
