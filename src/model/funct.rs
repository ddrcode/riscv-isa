use std::convert::TryFrom;
use std::fmt;

use crate::error::RISCVError;

/// A generic field type representing an instruction field with `BITS` bits.
/// The constant `SHIFT` indicates the bit position in the instruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunctField<const BITS: u8, const SHIFT: u8>(u8);

impl<const BITS: u8, const SHIFT: u8> FunctField<BITS, SHIFT> {
    /// Maximum allowed value given BITS (for an unsigned representation).
    const MAX_VALUE: u8 = (1 << BITS) - 1;
}

impl<const BITS: u8, const SHIFT: u8> TryFrom<u8> for FunctField<BITS, SHIFT> {
    type Error = RISCVError;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if bits > Self::MAX_VALUE {
            Err(RISCVError::InvalidFunctValue(BITS))
        } else {
            Ok(Self(bits))
        }
    }
}

impl<const BITS: u8, const SHIFT: u8> From<u32> for FunctField<BITS, SHIFT> {
    fn from(instr: u32) -> Self {
        let bits = ((instr >> SHIFT) & ((1 << BITS) - 1)) as u8;
        Self(bits)
    }
}

impl<const BITS: u8, const SHIFT: u8> From<FunctField<BITS, SHIFT>> for u32 {
    fn from(field: FunctField<BITS, SHIFT>) -> Self {
        u32::from(field.0) << SHIFT
    }
}

impl<const BITS: u8, const SHIFT: u8> From<FunctField<BITS, SHIFT>> for u8 {
    fn from(field: FunctField<BITS, SHIFT>) -> Self {
        field.0
    }
}

impl<const BITS: u8, const SHIFT: u8> fmt::Display for FunctField<BITS, SHIFT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0width$b}", self.0, width = BITS as usize)
    }
}

/// Type alias for Funct3
pub type Funct3 = FunctField<3, 12>;

/// Type alias for Funct7
pub type Funct7 = FunctField<7, 25>;
