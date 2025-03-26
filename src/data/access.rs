use super::{INSTRUCTIONS, SYSTEM_INSTRUCTIONS};
use crate::model::{Funct3, Funct7, Mnemonic, Opcode};

pub fn get_mnemonic(
    opcode: Opcode,
    funct3: Option<Funct3>,
    funct7: Option<Funct7>,
) -> Option<Mnemonic> {
    let op: u16 = (u8::from(opcode) >> 2).into();
    let f3: u16 = funct3.map_or(0, |val| u16::from(u8::from(val)));
    let f7: u16 = funct7.map_or(0, |val| u16::from(u8::from(val)));
    let code: u16 = op | (f3 << 5) | (f7 << 8);

    INSTRUCTIONS.get(&code).map(|res| res.0.into())
}

pub fn find_system_mnemonic(instr: u32) -> Option<Mnemonic> {
    SYSTEM_INSTRUCTIONS.get(&instr).map(|res| res.0.into())
}

pub fn find_instr_from_mnemonic(mnemonic: Mnemonic) -> Option<(Opcode, Funct3, Funct7)> {
    INSTRUCTIONS.iter().find_map(|(k, v)| {
        if Mnemonic::from(v.0) == mnemonic {
            let opcode = Opcode::try_from((0b11 | ((k & 0b11111) << 2)) as u8).unwrap();
            let funct3 = Funct3::try_from(((k >> 5) & 0b111) as u8).unwrap();
            let funct7 = Funct7::try_from((k >> 8) as u8).unwrap();
            Some((opcode, funct3, funct7))
        } else {
            None
        }
    })
}
