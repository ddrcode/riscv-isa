use super::{InstructionDef, INSTRUCTIONS, OFF, SYSTEM_INSTRUCTIONS};
use crate::model::{Funct3, Funct7, Mnemonic, Opcode};

pub fn get_mnemonic(
    opcode: Opcode,
    funct3: Option<Funct3>,
    funct7: Option<Funct7>,
) -> Option<Mnemonic> {
    let code: u16 = OFF::new(opcode, funct3, funct7).search_key();

    for def in INSTRUCTIONS.iter() {
        if let Some(key) = def.search_key {
            if key == code {
                return Some(def.mnemonic)
            }
        }
    }

    None
}

pub fn get_system_mnemonic(instr: u32) -> Option<Mnemonic> {
    SYSTEM_INSTRUCTIONS.get(&instr).map(|res| res.1)
}

pub fn get_instruction_from_mnemonic(mnemonic: &Mnemonic) -> Option<OFF> {
    let def = get_instruction_def(mnemonic);
    OFF::try_from(def).ok()
}

pub fn get_instruction_def(mnemonic: &Mnemonic) -> &InstructionDef {
    &INSTRUCTIONS[u16::from(mnemonic) as usize]
}
