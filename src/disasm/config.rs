use super::Address;

pub struct DisasmConfig {
    pub mnemonic_uppercase: bool,
    pub registry_uppercase: bool,

    pub show_addr: bool,
    pub start_addr: Address
}

impl Default for DisasmConfig {
    fn default() -> Self {
        Self {
            mnemonic_uppercase: false,
            registry_uppercase: false,

            show_addr: true,
            start_addr: 0
        }
    }
}
