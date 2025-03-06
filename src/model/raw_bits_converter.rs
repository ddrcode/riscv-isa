use crate::error::RISCVError;

pub trait RawBitsConverter {
    type BitsData;
    type Error;

    fn try_from_raw_bits(bits: Self::BitsData) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn into_raw_bits(&self) -> Self::BitsData;
}


