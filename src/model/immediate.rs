use std::fmt;
use crate::error::RISCVError;

pub struct Immediate<const SIZE: u8>(i32);

impl<const SIZE: u8> TryFrom<i32> for Immediate<SIZE> {
    type Error = RISCVError;

    fn try_from(imm: i32) -> Result<Self, Self::Error> {
        if imm > (1 << SIZE) {
            return Err(RISCVError::ImmediateTooBig(SIZE));
        }
        Ok(Immediate::<SIZE>(imm))
    }
}


impl<const SIZE: u8> fmt::Display for Immediate<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}
