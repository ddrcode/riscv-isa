use std::fmt;

use crate::data::get_instruction_def;

use super::Mnemonic;
// use crate::{config::UNKNOWN_MNEMONIC, data::INSTRUCTIONS};

// impl From<Mnemonic> for u16 {
//     fn from(mnemonic: Mnemonic) -> Self {
//         mnemonic.into()
//     }
// }
//
// impl From<&Mnemonic> for u16 {
//     fn from(mnemonic: &Mnemonic) -> Self {
//         mnemonic.into()
//     }
// }

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let def = get_instruction_def(self);
        write!(f, "{}", def.name)
    }
}
