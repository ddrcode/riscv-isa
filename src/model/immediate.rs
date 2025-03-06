use crate::error::RISCVError;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Immediate<const START: u8, const END: u8>(i32);

impl<const START: u8, const END: u8> TryFrom<i32> for Immediate<START, END> {
    type Error = RISCVError;

    fn try_from(imm: i32) -> Result<Self, Self::Error> {
        assert!(START <= END && END < 32, "Invalid immediate field bit positions");

        let width = END - START;
        let (max, min) = if END == 31 {
            (i32::MAX, i32::MIN)
        } else {
            ((1 << END) - 1, -(1 << END))
        };

        if imm > max || imm < min {
            return Err(RISCVError::ImmediateOutOfRange(min, max));
        }

        if START > 0 && (imm & ((1 << START) - 1)) != 0 {
            return Err(RISCVError::ImmediateBitsBeforeStart(START));
        }

        let trim = if width < 31 {
            (1 << (width + 1)) - 1
        } else {
            !0
        };
        let mut result = imm >> START;
        result &= trim;
        Ok(Immediate::<START, END>(result))
    }
}

impl<const START: u8, const END: u8> From<Immediate<START, END>> for i32 {
    fn from(imm: Immediate<START, END>) -> Self {
        let mut val = imm.0 << START;
        if END < 31 && val & (1 << (END)) != 0 {
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
        write!(f, "0x{:x}", i32::from(self))
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
        assert_imm::<1, 12>(1 << 12, Some(ImmediateOutOfRange(-4096, 4095)));
        assert_imm::<1, 12>(1, Some(ImmediateBitsBeforeStart(1)));

        assert_imm::<0, 11>(-1, None);
        assert_imm::<0, 11>(-2048, None);
        assert_imm::<1, 12>(-768, None);
        assert_imm::<1, 12>(-4096, None);

        assert_imm::<12, 31>(1 << 12, None);
        assert_imm::<12, 31>((1 << 12)+2048, Some(ImmediateBitsBeforeStart(12)));
        assert_imm::<12, 31>(-1, Some(ImmediateBitsBeforeStart(12)));
        assert_imm::<12, 31>(-65536, None);
    }
}
