use crate::error::RISCVError;

pub struct Immediate<const SIZE: u8>(u32);

impl TryFrom<u32> for Immediate<12> {
    type Error = RISCVError;

    fn try_from(imm: u32) -> Result<Self, Self::Error> {
        Ok(Immediate::<12>(1))
    }
}
