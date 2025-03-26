use std::fmt::Write;

use crate::{
    data::find_system_mnemonic,
    instr::{Instruction, InstructionTrait},
    model::{Mnemonic, Register},
};

use super::{Address, DisasmConfig, InstructionRecord};

/// A formatter for instructions, providing various formatting options based on
/// provided configuration.
#[derive(Debug, Clone)]
pub struct InstructionFormatter {
    config: DisasmConfig,
}

impl InstructionFormatter {
    /// Creates a new `InstructionFormatter` with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the formatter.
    pub fn new(config: DisasmConfig) -> InstructionFormatter {
        InstructionFormatter { config }
    }

    /// Formats an optional mnemonic. If `None` (which means unrecognized mnemonic)
    /// it returns a string specified by `self.config.unknown_mnemonic`
    ///
    /// # Arguments
    ///
    /// * `m` - An optional mnemonic to format.
    ///
    /// # Returns
    ///
    /// A formatted mnemonic string.
    pub fn optional_mnemonic(&self, m: Option<Mnemonic>) -> String {
        m.map_or(self.config.unknown_mnemonic.clone(), |m| {
            if self.config.mnemonic_uppercase {
                m.to_string().to_uppercase()
            } else {
                m.to_string().to_lowercase()
            }
        })
    }

    /// Formats a mnemonic.
    ///
    /// # Arguments
    ///
    /// * `m` - The mnemonic to format.
    ///
    /// # Returns
    ///
    /// A formatted mnemonic string.
    pub fn mnemonic(&self, m: &Mnemonic) -> String {
        if self.config.mnemonic_uppercase {
            m.to_string().to_uppercase()
        } else {
            m.to_string().to_lowercase()
        }
    }

    /// Formats an instruction.
    ///
    /// # Arguments
    ///
    /// * `instr` - a reference to an instruction to format.
    ///
    /// # Returns
    ///
    /// A formatted instruction string.
    pub fn instruction(&self, instr: &Instruction) -> String {
        use Instruction::*;
        let mut out = format!(
            "{}{}",
            self.optional_mnemonic(instr.mnemonic()),
            self.config.mnemonic_separator
        );

        let s = &self.config.register_separator;
        let result = match instr {
            R(i) => write!(
                out,
                "{},{}{},{}{}",
                self.register(&i.rd()),
                s,
                self.register(&i.rs1()),
                s,
                self.register(&i.rs2())
            ),

            I(i) => {
                if find_system_mnemonic(instr.into()).is_some() {
                    Ok(())
                } else {
                    write!(
                        out,
                        "{},{}{},{}{}",
                        self.register(&i.rd()),
                        s,
                        self.register(&i.rs1()),
                        s,
                        self.number(i.imm().into())
                    )
                }
            }

            S(i) => write!(
                out,
                "{},{}{}({})",
                self.register(&i.rs2()),
                s,
                self.number(i.imm().into()),
                self.register(&i.rs1())
            ),

            B(i) => write!(
                out,
                "{},{}{},{}{}",
                self.register(&i.rs1()),
                s,
                self.register(&i.rs2()),
                s,
                self.number(i.imm().into())
            ),

            U(i) => write!(
                out,
                "{},{}{}",
                self.register(&i.rd()),
                s,
                self.number(i.imm().into())
            ),

            J(i) => write!(
                out,
                "{},{}{}",
                self.register(&i.rd()),
                s,
                self.number(i.imm().into())
            ),
        };
        if result.is_err() {
            todo!();
        }
        out
    }

    /// Formats a register.
    ///
    /// # Arguments
    ///
    /// * `r` - The register to format.
    ///
    /// # Returns
    ///
    /// A formatted register string.
    pub fn register(&self, r: &Register) -> String {
        if self.config.register_uppercase {
            r.to_string().to_uppercase()
        } else {
            r.to_string().to_lowercase()
        }
    }

    /// Formats a number.
    ///
    /// # Arguments
    ///
    /// * `n` - The number to format.
    ///
    /// # Returns
    ///
    /// A formatted number string.
    pub fn number(&self, n: i32) -> String {
        (self.config.immediate_format)(n)
    }

    /// Formats an address.
    ///
    /// # Arguments
    ///
    /// * `a` - The address to format.
    ///
    /// # Returns
    ///
    /// A formatted address string.
    pub fn address(&self, a: Address) -> String {
        format!("0x{:08x}", a)
    }

    /// Formats an instruction record.
    ///
    /// # Arguments
    ///
    /// * `r` - The instruction record to format.
    ///
    /// # Returns
    ///
    /// A formatted instruction record string.
    pub fn record(&self, r: &InstructionRecord) -> String {
        let instr = self.instruction(&r.instruction());
        if self.config.show_addr {
            format!(
                "{}{}{}",
                self.address(r.address()),
                self.config.addr_separator,
                instr
            )
        } else {
            instr
        }
    }
}
