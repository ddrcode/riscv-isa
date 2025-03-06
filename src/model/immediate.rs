use std::fmt;

use super::RawBitsConverter;
use crate::error::RISCVError;

#[derive(Debug, Copy, Clone)]
pub struct Immediate<const START: u8, const END: u8>(i32);

impl<const START: u8, const END: u8> TryFrom<i32> for Immediate<START, END> {
    type Error = RISCVError;

    fn try_from(imm: i32) -> Result<Self, Self::Error> {
        debug_assert!(
            START <= END && END < 32,
            "Invalid immediate field bit positions"
        );

        // The total field width (number of bits) is:
        let field_width = END - START + 1;
        // The number of magnitude bits (excluding the sign bit)
        let mag_bits = field_width - 1;

        // Compute the valid range for a two's complement field of width field_width
        let (max, min) = if END == 31 {
            (i32::MAX, i32::MIN)
        } else {
            (((1 << mag_bits) - 1) << START, -(1 << mag_bits) << START)
        };

        if imm > max || imm < min {
            return Err(RISCVError::ImmediateOutOfRange(min, max));
        }

        // For fields with a nonzero START, the lower START bits must be zero.
        if START > 0 && (imm & ((1 << START) - 1)) != 0 {
            return Err(RISCVError::ImmediateBitsBeforeStart(START));
        }

        // Shift right by START to normalize the field into lower bits.
        let mut result = imm >> START;

        // Zero all bits from outside of the range
        let mask = if field_width < 32 {
            (1 << field_width) - 1
        } else {
            !0
        };
        result &= mask;

        Ok(Immediate::<START, END>(result))
    }
}

impl<const START: u8, const END: u8> RawBitsConverter<u32> for Immediate<START, END> {
    type Error = RISCVError;

    fn try_from_raw_bits(bits: u32) -> Result<Self, RISCVError> {
        debug_assert!(
            START <= END && END < 32,
            "Invalid immediate field bit positions"
        );

        let width = END - START + 1;
        if width < 31 && bits > ((1 << width) - 1) {
            return Err(RISCVError::ImmediateOutOfRange(0, 0));
        }

        Ok(Immediate::<START, END>(i32::from_le_bytes(
            bits.to_le_bytes(),
        )))
    }

    fn into_raw_bits(&self) -> u32 {
        u32::from_le_bytes(self.0.to_le_bytes())
    }
}

impl<const START: u8, const END: u8> From<Immediate<START, END>> for i32 {
    fn from(imm: Immediate<START, END>) -> Self {
        let mut val = imm.0 << START;
        if END < 31 && (val & (1 << END)) != 0 {
            val |= !((1 << END) - 1);
        }
        val
    }
}

impl<const START: u8, const END: u8> From<&Immediate<START, END>> for i32 {
    fn from(imm: &Immediate<START, END>) -> Self {
        i32::from(*imm)
    }
}

impl<const START: u8, const END: u8> fmt::Display for Immediate<START, END> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:x}", self.into_raw_bits())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assert_imm<const START: u8, const END: u8>(val: i32, err: Option<RISCVError>) {
        let result = Immediate::<START, END>::try_from(val);
        match err {
            Some(e_exp) => match result {
                Ok(_) => assert!(false, "Immediate creation expected to fail for {}", val),
                Err(e) => assert_eq!(e_exp, e),
            },
            None => match result {
                Ok(imm) => assert_eq!(val, imm.into()),
                Err(e) => assert!(
                    false,
                    "Immediate creation expected to work for {}, but it failed with {} instead",
                    val, e
                ),
            },
        }
    }

    fn assert_bit_num<const START: u8, const END: u8>(bits: u32, exp: i32) {
        let imm = Immediate::<START, END>::try_from_raw_bits(bits).unwrap();
        assert_eq!(exp, imm.into())
    }

    #[test]
    fn test_two_way_conversion() {
        use RISCVError::*;

        assert_imm::<0, 11>(10, None);
        assert_imm::<0, 11>(2047, None);
        assert_imm::<0, 11>((1 << 11) - 1, None);
        assert_imm::<0, 11>(1 << 11, Some(ImmediateOutOfRange(-2048, 2047)));

        assert_imm::<1, 12>(0, None);
        assert_imm::<1, 12>(10, None);
        assert_imm::<1, 12>(2046, None);
        assert_imm::<1, 12>((1 << 11) - 2, None);
        assert_imm::<1, 12>(1 << 12, Some(ImmediateOutOfRange(-4096, 4094)));
        assert_imm::<1, 12>(1, Some(ImmediateBitsBeforeStart(1)));

        assert_imm::<0, 11>(-1, None);
        assert_imm::<0, 11>(-2048, None);
        assert_imm::<1, 12>(-768, None);
        assert_imm::<1, 12>(-4096, None);

        assert_imm::<12, 31>(1 << 12, None);
        assert_imm::<12, 31>((1 << 12) + 2048, Some(ImmediateBitsBeforeStart(12)));
        assert_imm::<12, 31>(-1, Some(ImmediateBitsBeforeStart(12)));
        assert_imm::<12, 31>(-65536, None);
    }

    #[test]
    fn test_bits_to_num() {
        assert_bit_num::<0, 11>(5, 5);
        assert_bit_num::<0, 11>(0b111111100111, -25);
        assert_bit_num::<12, 31>(1, 4096);
    }
}
