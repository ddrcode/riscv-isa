use std::fmt::Write;

use crate::model::{Mnemonic, Register};

use super::{Address, DisasmConfig, InstructionRecord};

#[derive(Debug, Clone)]
pub struct InstructionFormatter {
    config: DisasmConfig,
}

impl InstructionFormatter {
    pub fn new(config: DisasmConfig) -> InstructionFormatter {
        InstructionFormatter { config }
    }

    pub fn optional_mnemonic(&self, m: Option<Mnemonic>) -> String {
        m.map_or(self.config.unknown_mnemonic.clone(), |m| {
            if self.config.mnemonic_uppercase {
                m.to_string().to_uppercase()
            } else {
                m.to_string().to_lowercase()
            }
        })
    }

    pub fn mnemonic(&self, m: &Mnemonic) -> String {
        if self.config.mnemonic_uppercase {
            m.to_string().to_uppercase()
        } else {
            m.to_string().to_lowercase()
        }
    }

    pub fn register(&self, r: &Register) -> String {
        if self.config.register_uppercase {
            r.to_string().to_uppercase()
        } else {
            r.to_string().to_lowercase()
        }
    }

    pub fn address(&self, a: Address) -> String {
        format!("0x{:08x}", a)
    }

    pub fn record(&self, r: &InstructionRecord) -> String {
        format!("{}", self.address(r.address()))
    }
}
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn test_string() {
//         let mut f = InstructionFormatter::new(DisasmConfig::default());
//         let m = f.mnemonic(Some("koza"));
//         println!("{}", m);
//     }
// }
