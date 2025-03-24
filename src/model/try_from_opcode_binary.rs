use crate::error::RISCVError;

pub trait TryFromOpcodeBinary {
    fn try_from_opcode_binary(bits: u8) -> Result<Self, RISCVError>
    where
        Self: Sized;
}
