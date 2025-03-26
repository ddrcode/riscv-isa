use std::fmt;


#[derive(Debug, PartialEq)]
pub struct Mnemonic(&'static str);

impl From<&'static str> for Mnemonic {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl From<Mnemonic> for &'static str {
    fn from(value: Mnemonic) -> Self {
        value.0
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
