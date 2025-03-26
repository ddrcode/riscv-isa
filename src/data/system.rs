use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::model::RISCVExtension as EXT;

type Row = (&'static str, EXT, u8);

pub(crate) static SYSTEM_INSTRUCTIONS: Lazy<HashMap<u32, Row>> = Lazy::new(|| {
    HashMap::from([
        (0x73, ("ecall", EXT::I, 32)),
        (0x100073, ("ebreak", EXT::I, 32)),
        (0x10500073, ("wfi", EXT::System, 32)),
        (0x30200073, ("mret", EXT::System, 32)),
        (0x10200073, ("sret", EXT::System, 32)),
        (0x7b200073, ("dret", EXT::Sdext, 32)),
    ])
});
