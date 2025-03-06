use crate::error::RISCVError;
use std::fmt;

pub struct Immediate<const START: u8, const END: u8>(i32);

impl<const START: u8, const END: u8> TryFrom<i32> for Immediate<START, END> {
    type Error = RISCVError;

    fn try_from(imm: i32) -> Result<Self, Self::Error> {
        let width = END - START;
        let max = (1 << width) - 1;
        let min = -(1 << width);

        if imm > max || imm < min {
            return Err(RISCVError::ImmediateOutOfRange(min, max))
        }

        // if END < 31 && imm > (1 << END) - 1 {
        //     return Err(RISCVError::ImmediateTooBig(END));
        // }
        if START > 0 && (imm & ((1 << START) - 1)) != 0 {
            return Err(RISCVError::ImmediateBitsBeforeStart(START));
        }

        // let trim = if END < 31 { (1 << (END+1)) - 1 } else { imm };
        // let sign = if imm >= 0 { 0 } else { 1 << width };
        //

        let trim = if width < 31 { (1 << (width+1)) - 1 } else { !0 };
        let mut result = imm >> START;
        result &= trim;
        Ok(Immediate::<START, END>(result))
    }
}

impl<const START: u8, const END: u8> From<Immediate<START, END>> for i32 {
    fn from(imm: Immediate<START, END>) -> Self {
        let mut val = imm.0 << START;
        if val & (1 << (END)) != 0 {
            val = !val + 1;
        }
        val
    }
}

impl<const START: u8, const END: u8> From<&Immediate<START, END>> for i32 {
    fn from(imm: &Immediate<START, END>) -> Self {
        let mut val = imm.0 << START;
        if val & (1 << (END)) != 0 {
            val = !val + 1;
        }
        val
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
                Err(_) => assert!(false, "Immediate creation expected to work for {}", val),
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
        //
        assert_imm::<1, 12>(0, None);
        assert_imm::<1, 12>(10, None);
        assert_imm::<1, 12>(2046, None);
        assert_imm::<1, 12>((1 << 11) - 2, None);
        assert_imm::<1, 12>(1 << 12, Some(ImmediateOutOfRange(-2048, 2047)));
        assert_imm::<1, 12>(1, Some(ImmediateBitsBeforeStart(1)));
        //
        assert_imm::<0, 11>(-1, None);
        // assert_imm::<1, 12>(-768, None);
    }
}
