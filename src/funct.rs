use crate::error::RISCVError;


#[derive(Debug, Clone, Copy)]
pub struct Funct3(u8);

impl TryFrom<u8> for Funct3 {
    type Error = RISCVError;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if bits & 0b11111000 > 0 {
            return Err(RISCVError::InvalidFunct3Value);
        }
        Ok(Self(bits))
    }
}


impl From<u32> for Funct3 {
    fn from(instr: u32) -> Self {
        let bits = match u8::try_from((instr >> 12) & 0b111) {
            Ok(val) => val,
            Err(_) => unreachable!()
        };
        Self(bits)
    }
}

impl From<Funct3> for u32 {
    fn from(funct3: Funct3) -> Self {
        u32::from(funct3.0) << 12
    }
}

impl From<Funct3> for u8 {
    fn from(funct3: Funct3) -> Self {
        match (u32::from(funct3) >> 12).try_into() {
            Ok(val) => val,
            Err(_) => unreachable!()
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Funct7(u8);

impl TryFrom<u8> for Funct7 {
    type Error = RISCVError;

    fn try_from(bits: u8) -> Result<Self, Self::Error> {
        if bits > 127 {
            return Err(RISCVError::InvalidFunct7Value);
        }
        Ok(Self(bits))
    }
}


impl From<u32> for Funct7 {
    fn from(instr: u32) -> Self {
        let bits = match u8::try_from(instr >> 25) {
            Ok(val) => val,
            Err(_) => unreachable!()
        };
        Self(bits)
    }
}

impl From<Funct7> for u32 {
    fn from(funct7: Funct7) -> Self {
        u32::from(funct7.0) << 25
    }
}


impl From<Funct7> for u8 {
    fn from(funct7: Funct7) -> Self {
        match (u32::from(funct7) >> 25).try_into() {
            Ok(val) => val,
            Err(_) => unreachable!()
        }
    }
}
