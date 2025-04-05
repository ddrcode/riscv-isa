use crate::model::{Mnemonic, RISCVExtension};

static KOZA: [u8; 3] = [1,2,3];

pub struct InstructionDef {
    pub name: String,
    pub mnemonic: Mnemonic,
    pub extension: RISCVExtension,
    pub arch: u8,
    pub mask: u32,
    pub match_val: u32,
    pub search_key: Option<u16>,
}

impl InstructionDef {
    pub fn new(
        name: &str,
        mnemonic: Mnemonic,
        extension: RISCVExtension,
        arch: u8,
        mask: u32,
        match_val: u32,
        search_key: Option<u16>,
    ) -> Self {
        Self {
            name: name.to_string(),
            mnemonic,
            extension,
            arch,
            mask,
            match_val,
            search_key,
        }
    }
}
