use crate::config::UNKNOWN_MNEMONIC;

use super::Address;

#[derive(Debug, Clone, PartialEq)]
pub struct DisasmConfig {
    pub mnemonic_uppercase: bool,
    pub mnemonic_separator: String,

    pub register_uppercase: bool,
    pub register_separator: String,

    pub immediate_format: fn(i32) -> String,

    pub show_addr: bool,
    pub start_addr: Address,
    pub addr_format: String,

    pub unknown_mnemonic: String,
    pub hex_uppercase: bool,
}

impl DisasmConfig {
    pub fn with_address(addr: Address) -> Self {
        Self {
            start_addr: addr,
            ..Default::default()
        }
    }
}

impl Default for DisasmConfig {
    fn default() -> Self {
        Self {
            mnemonic_uppercase: false,
            mnemonic_separator: String::from(" "),

            register_uppercase: false,
            register_separator: String::from(" "),

            immediate_format: |_| String::from("{:x}"),

            show_addr: true,
            start_addr: 0,
            addr_format: String::from("{:x}"),

            unknown_mnemonic: UNKNOWN_MNEMONIC.to_string(),
            hex_uppercase: false,
        }
    }
}
