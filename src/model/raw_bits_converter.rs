pub trait RawBitsConverter<T> {
    type Error;

    fn try_from_raw_bits(bits: T) -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn into_raw_bits(&self) -> T;
}


