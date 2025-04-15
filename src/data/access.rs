use super::{InstructionDef, INSTRUCTIONS };
use crate::model::{Mnemonic, Opff};
use crate::instr::InstructionTrait;

pub fn get_mnemonic<T: InstructionTrait + From<T> + Copy>(instr: &T) -> Option<Mnemonic> {
    let mnem = get_mnemonic_from_opff(&instr.opff());
    if mnem.is_some() {
        return mnem;
    }

    let _code: u32 = <T as Into<T>>::into(*instr).into();
    None
}

pub fn get_mnemonic_from_opff(opff: &Opff) -> Option<Mnemonic> {
    let code: u16 = opff.search_key();

    for def in INSTRUCTIONS.iter() {
        if let Some(key) = def.search_key {
            if key == code {
                return Some(def.mnemonic);
            }
        }
    }

    None
}

pub fn get_off_from_mnemonic(mnemonic: &Mnemonic) -> Option<Opff> {
    let def = get_instruction_def(mnemonic);
    Opff::try_from(def).ok()
}

pub fn get_instruction_def(mnemonic: &Mnemonic) -> &InstructionDef {
    &INSTRUCTIONS[u16::from(mnemonic) as usize]
}
