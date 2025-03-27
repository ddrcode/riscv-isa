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

    INSTRUCTIONS.get(&code).map(|res| res.1)
}

pub fn find_system_mnemonic(instr: u32) -> Option<Mnemonic> {
    SYSTEM_INSTRUCTIONS.get(&instr).map(|res| res.1)
}

