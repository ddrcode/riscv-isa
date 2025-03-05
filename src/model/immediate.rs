use std::fmt;
use crate::error::RISCVError;

pub struct Immediate<const SIZE: u8>(u32);

impl<const SIZE: u8> TryFrom<u32> for Immediate<SIZE> {
    type Error = RISCVError;

    fn try_from(imm: u32) -> Result<Self, Self::Error> {
        if imm > (1 << SIZE) {
            return Err(RISCVError::ImmediateTooBig(SIZE));
        }
        Ok(Immediate::<SIZE>(1))
    }
}


impl<const SIZE: u8> fmt::Display for Immediate<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
