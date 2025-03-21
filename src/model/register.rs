use std::fmt;
use crate::error::RISCVError;

pub const REGISTER_MASK: u32 = 0b11111;

#[derive(Debug)]
pub struct Register(u8);

impl Register {
    fn from_instr_bits(instr: u32, shift: u32) -> Self {
        let bits = (instr >> shift) & REGISTER_MASK;
        match Self::try_from(bits as u8) {
            Ok(reg) => reg,
            Err(_) => unreachable!()
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
            _ => unreachable!()
        };
        write!(f, "{}", s)
    }
}
