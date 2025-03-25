use crate::error::RISCVError;
use std::fmt;

pub const REGISTER_MASK: u32 = 0b11111;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Register(u8);

macro_rules! register_constructors {
    ($($name:ident => $index:expr),*) => {
        $(
            #[doc = concat!("Constructor for `", stringify!($name), "` register")]
            pub fn $name() -> Self { Self($index) }
        )*
    };
}

impl Register {
    register_constructors! {
        zero => 0,
        ra => 1,
        sp => 2,
        gp => 3,
        tp => 4,
        t0 => 5,
        t1 => 6,
        t2 => 7,
        s0 => 8,
        s1 => 9,
        a0 => 10,
        a1 => 11,
        a2 => 12,
        a3 => 13,
        a4 => 14,
        a5 => 15,
        a6 => 16,
        a7 => 17,
        s2 => 18,
        s3 => 19,
        s4 => 20,
        s5 => 21,
        s6 => 22,
        s7 => 23,
        s8 => 24,
        s9 => 25,
        s10 => 26,
        s11 => 27,
        t3 => 28,
        t4 => 29,
        t5 => 30,
        t6 => 31
    }

    fn from_instr_bits(instr: u32, shift: u32) -> Self {
        let bits = (instr >> shift) & REGISTER_MASK;
        match Self::try_from(bits as u8) {
            Ok(reg) => reg,
            Err(_) => unreachable!("As the value is masked, it's always 5-bits, as expected"),
        }
    }

    pub fn from_rs1_bits(instr: u32) -> Self {
        Self::from_instr_bits(instr, 15)
    }

    pub fn from_rs2_bits(instr: u32) -> Self {
        Self::from_instr_bits(instr, 20)
    }

    pub fn from_rd_bits(instr: u32) -> Self {
        Self::from_instr_bits(instr, 7)
    }

    pub fn into_rs1_bits(&self) -> u32 {
        u32::from(self) << 15
    }

    pub fn into_rs2_bits(&self) -> u32 {
        u32::from(self) << 20
    }

    pub fn into_rd_bits(&self) -> u32 {
        u32::from(self) << 7
    }
}

impl TryFrom<u8> for Register {
    type Error = RISCVError;

    fn try_from(reg: u8) -> Result<Self, Self::Error> {
        if reg < 32 {
            Ok(Self(reg))
        } else {
            Err(RISCVError::InvalidRegister)
        }
    }
}

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        reg.0
    }
}

impl From<&Register> for u8 {
    fn from(reg: &Register) -> Self {
        reg.0
    }
}

impl From<Register> for u32 {
    fn from(reg: Register) -> Self {
        reg.0.into()
    }
}

impl From<&Register> for u32 {
    fn from(reg: &Register) -> Self {
        reg.0.into()
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self.0 {
            0 => "zero",
            1 => "ra",
            2 => "sp",
            3 => "gp",
            4 => "tp",
            5 => "t0",
            6 => "t1",
            7 => "t2",
            8 => "s0",
            9 => "s1",
            10 => "a0",
            11 => "a1",
            12 => "a2",
            13 => "a3",
            14 => "a4",
            15 => "a5",
            16 => "a6",
            17 => "a7",
            18 => "s2",
            19 => "s3",
            20 => "s4",
            21 => "s5",
            22 => "s6",
            23 => "s7",
            24 => "s8",
            25 => "s9",
            26 => "s10",
            27 => "s11",
            28 => "t3",
            29 => "t4",
            30 => "t5",
            31 => "t6",
            _ => unreachable!(),
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u8() {
        assert!(Register::try_from(0).is_ok());
        assert!(Register::try_from(31).is_ok());
        assert!(Register::try_from(32).is_err());
        assert!(Register::try_from(255).is_err());
    }

    #[test]
    fn test_from_rs1_bits() {
        let instr: u32 = 13 << 15;
        let reg = Register::from_rs1_bits(instr);
        assert_eq!(reg, Register::a3());
    }

    #[test]
    fn test_into_rs1_bits() {
        let reg = Register::ra();
        let bits = reg.into_rs1_bits();
        assert_eq!(bits, 1 << 15);
    }
}
