use std::fmt;

use super::Mnemonic;
// use crate::{config::UNKNOWN_MNEMONIC, data::INSTRUCTIONS};

impl From<Mnemonic> for u16 {
    fn from(mnemonic: Mnemonic) -> Self {
        mnemonic.into()
    }
}

impl From<&Mnemonic> for u16 {
    fn from(mnemonic: &Mnemonic) -> Self {
        mnemonic.into()
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
