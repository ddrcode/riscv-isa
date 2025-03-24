use std::fmt::Write;

use crate::{
    instr::{Instruction, InstructionTrait},
    model::{Mnemonic, Register},
};

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

    pub fn instruction(&self, instr: &Instruction) -> String {
        use Instruction::*;
        let mut out = format!(
            "{}{}",
            self.optional_mnemonic(instr.get_mnemonic()),
            self.config.mnemonic_separator
        );
        let s = &self.config.register_separator;
        let _ = match instr {
            R(i) => write!(
                out,
                "{},{}{},{}{}",
                self.register(&i.rd()),
                s,
                self.register(&i.rs1()),
                s,
                self.register(&i.rs2())
            ),
            I(i) => write!(
                out,
                "{},{}{},{}{}",
                self.register(&i.rd()),
                s,
                self.register(&i.rs1()),
                s,
                self.number(i.imm().into())
            ),
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
        out
    }

    pub fn register(&self, r: &Register) -> String {
        if self.config.register_uppercase {
            r.to_string().to_uppercase()
        } else {
            r.to_string().to_lowercase()
        }
    }

    pub fn number(&self, n: i32) -> String {
        format!("{:x}", n)
    }

    pub fn address(&self, a: Address) -> String {
        format!("0x{:08x}", a)
    }

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
