use crate::{
    model::{Funct3, Funct7, Opcode, FUNCT3_MASK, FUNCT7_MASK, OPCODE_MASK},
    RISCVError,
};

use super::InstructionDef;

#[derive(Debug)]
pub struct OFF {
    opcode: Opcode,
    funct3: Option<Funct3>,
    funct7: Option<Funct7>,
}

impl OFF {
    pub fn new(opcode: Opcode, funct3: Option<Funct3>, funct7: Option<Funct7>) -> Self {
        OFF {
            opcode,
            funct3,
            funct7,
        }
    }

    pub fn opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn funct3(&self) -> Option<Funct3> {
        self.funct3
    }

    pub fn funct7(&self) -> Option<Funct7> {
        self.funct7
    }

    pub fn mask(&self) -> u32 {
        let mut m: u32 = OPCODE_MASK;
        if self.funct3.is_some() {
            m |= FUNCT3_MASK;
        }
        if self.funct7.is_some() {
            m |= FUNCT7_MASK;
        }
        m
    }

    pub fn match_val(&self) -> u32 {
        u32::from(self.opcode) | self.funct3.map_or(0, u32::from) | self.funct7.map_or(0, u32::from)
    }

    pub fn search_key(&self) -> u16 {
        let op: u16 = (u8::from(self.opcode) >> 2).into();
        let f3: u16 = self.funct3.map_or(0, u8::from).into();
        let f7: u16 = self.funct7.map_or(0, u8::from).into();
        op | (f3 << 5) | (f7 << 8)
    }
}

impl TryFrom<&InstructionDef> for OFF {
    type Error = RISCVError;

    fn try_from(def: &InstructionDef) -> Result<Self, Self::Error> {
        if def.mask & OPCODE_MASK != OPCODE_MASK {
            return Err(RISCVError::OFFError(format!(
                "Can't determine opcode for instruction {0}",
                def.name
            )));
        }

        let funct3 = if def.mask & FUNCT3_MASK == FUNCT3_MASK {
            Some(Funct3::from(def.match_val))
        } else {
            None
        };

        let funct7 = if def.mask & FUNCT7_MASK == FUNCT7_MASK {
            Some(Funct7::from(def.match_val))
        } else {
            None
        };

        Ok(OFF {
            opcode: Opcode::try_from(def.match_val)?,
            funct3,
            funct7,
        })
    }
}

impl TryFrom<InstructionDef> for OFF {
    type Error = RISCVError;

    fn try_from(def: InstructionDef) -> Result<Self, Self::Error> {
        OFF::try_from(&def)
    }
}
