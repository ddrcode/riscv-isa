use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::model::{RISCVExtension as EXT, Mnemonic as M};

type Row = (&'static str, M, u16, EXT, u8);

pub(crate) static SYSTEM_INSTRUCTIONS: Lazy<HashMap<u32, Row>> = Lazy::new(|| {
    HashMap::from([
        (0x73, ("ecall", M::Ecall, 0xfff1, EXT::I, 32)),
        (0x100073, ("ebreak", M::Ebreak, 0xfff2, EXT::I, 32)),
        (0x10500073, ("wfi", M::Wfi, 0xfff3  , EXT::System, 32)),
        (0x30200073, ("mret", M::Mret, 0xfff4, EXT::System, 32)),
        (0x10200073, ("sret", M::Sret, 0xfff5, EXT::System, 32)),
        (0x7b200073, ("dret", M::Dret, 0xfff6, EXT::Sdext, 32)),
    ])
});
