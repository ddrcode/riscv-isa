use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::model::{Funct3, Funct7, InstructionFormat as IF, Mnemonic, Opcode, RISCVExtension as EXT};

type Row = (&'static str, IF, EXT, u8);

static INSTRUCTIONS: Lazy<HashMap<u16, Row>> = Lazy::new(|| {
    HashMap::from([
        (0b01011_010_0000000, ("LR.W", IF::R, EXT::A, 32)),
        (0b01011_010_0000001, ("SC.W", IF::R, EXT::A, 32)),
        (
            0b01011_011_0000000,
            ("AMOSWAP.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_0000001,
            ("AMOADD.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_0010000,
            ("AMOXOR.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_0110000,
            ("AMOAND.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_0100000,
            ("AMOOR.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_1000000,
            ("AMOMIN.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_1010000,
            ("AMOMAX.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_1100000,
            ("AMOMINU.W", IF::R, EXT::A, 32),
        ),
        (
            0b01011_011_1110000,
            ("AMOMAXU.W", IF::R, EXT::A, 32),
        ),
        (0b10100_001_0000000, ("FLD", IF::I, EXT::D, 64)),
        (0b10100_011_0000000, ("FSD", IF::S, EXT::D, 64)),
        (0b10100_000_0000000, ("FLW", IF::I, EXT::F, 32)),
        (0b10100_010_0000000, ("FSW", IF::S, EXT::F, 32)),
        (
            0b11110_000_0000000,
            ("HLV.W", IF::I, EXT::H, 64),
        ),
        (
            0b11110_000_0000001,
            ("HSV.W", IF::S, EXT::H, 64),
        ),
        (0b01101_000_0000000, ("LUI", IF::U, EXT::I, 32)),
        (
            0b00101_000_0000000,
            ("AUIPC", IF::U, EXT::I, 32),
        ),
        (0b11011_000_0000000, ("JAL", IF::J, EXT::I, 32)),
        (0b01001_000_0000000, ("JALR", IF::I, EXT::I, 32)),
        (0b01000_000_0000000, ("BEQ", IF::B, EXT::I, 32)),
        (0b01000_001_0000000, ("BNE", IF::B, EXT::I, 32)),
        (0b01000_100_0000000, ("BLT", IF::B, EXT::I, 32)),
        (0b01000_101_0000000, ("BGE", IF::B, EXT::I, 32)),
        (0b01000_110_0000000, ("BLTU", IF::B, EXT::I, 32)),
        (0b01000_111_0000000, ("BGEU", IF::B, EXT::I, 32)),
        (0b00000_000_0000000, ("LB", IF::I, EXT::I, 32)),
        (0b00000_001_0000000, ("LH", IF::I, EXT::I, 32)),
        (0b00000_010_0000000, ("LW", IF::I, EXT::I, 32)),
        (0b00000_100_0000000, ("LBU", IF::I, EXT::I, 32)),
        (0b00000_101_0000000, ("LHU", IF::I, EXT::I, 32)),
        (0b01000_000_0000000, ("SB", IF::S, EXT::I, 32)),
        (0b01000_001_0000000, ("SH", IF::S, EXT::I, 32)),
        (0b01000_010_0000000, ("SW", IF::S, EXT::I, 32)),
        (0b00100_000_0000000, ("ADDI", IF::I, EXT::I, 32)),
        (0b00100_010_0000000, ("SLTI", IF::I, EXT::I, 32)),
        (
            0b00100_011_0000000,
            ("SLTIU", IF::I, EXT::I, 32),
        ),
        (0b00100_100_0000000, ("XORI", IF::I, EXT::I, 32)),
        (0b00100_110_0000000, ("ORI", IF::I, EXT::I, 32)),
        (0b00100_111_0000000, ("ANDI", IF::I, EXT::I, 32)),
        (0b00100_001_0000000, ("SLLI", IF::I, EXT::I, 32)),
        (0b00100_101_0000000, ("SRLI", IF::I, EXT::I, 32)),
        (0b00100_101_0100000, ("SRAI", IF::I, EXT::I, 32)),
        (0b01100_000_0000000, ("ADD", IF::R, EXT::I, 32)),
        (0b01100_000_0100000, ("SUB", IF::R, EXT::I, 32)),
        (0b01100_001_0000000, ("SLL", IF::R, EXT::I, 32)),
        (0b01100_010_0000000, ("SLT", IF::R, EXT::I, 32)),
        (0b01100_011_0000000, ("SLTU", IF::R, EXT::I, 32)),
        (0b01100_100_0000000, ("XOR", IF::R, EXT::I, 32)),
        (0b01100_101_0000000, ("SRL", IF::R, EXT::I, 32)),
        (0b01100_101_0100000, ("SRA", IF::R, EXT::I, 32)),
        (0b01100_110_0000000, ("OR", IF::R, EXT::I, 32)),
        (0b01100_111_0000000, ("AND", IF::R, EXT::I, 32)),
        (
            0b11100_000_0000000,
            ("ECALL", IF::I, EXT::I, 32),
        ),
        (
            0b11100_000_0000001,
            ("EBREAK", IF::I, EXT::I, 32),
        ),
        (0b01100_000_0000001, ("MUL", IF::R, EXT::M, 32)),
        (0b01100_001_0000001, ("MULH", IF::R, EXT::M, 32)),
        (
            0b01100_010_0000001,
            ("MULHSU", IF::R, EXT::M, 32),
        ),
        (
            0b01100_011_0000001,
            ("MULHU", IF::R, EXT::M, 32),
        ),
        (0b01100_100_0000001, ("DIV", IF::R, EXT::M, 32)),
        (0b01100_101_0000001, ("DIVU", IF::R, EXT::M, 32)),
        (0b01100_110_0000001, ("REM", IF::R, EXT::M, 32)),
        (0b01100_111_0000001, ("REMU", IF::R, EXT::M, 32)),
        (0b10100_011_0000001, ("FLQ", IF::I, EXT::Q, 64)),
        (0b10100_011_0000011, ("FSQ", IF::S, EXT::Q, 64)),
        (
            0b11110_000_0000010,
            ("SSTC.SET", IF::I, EXT::Sstc, 64),
        ),
        (
            0b11101_000_0000000,
            ("VADDR.TRANS", IF::I, EXT::Svadu, 64),
        ),
        (
            0b00000_000_0001000,
            ("PAUSE.W", IF::I, EXT::Zawrs, 32),
        ),
        (
            0b00010_001_0000000,
            ("CBO.FLUSH", IF::I, EXT::Zicbom, 32),
        ),
        (
            0b00010_000_0000000,
            ("CBO.ZERO", IF::I, EXT::Zicboz, 32),
        ),
        (
            0b11000_000_0000000,
            ("RDCYCLE", IF::I, EXT::Zicntr, 32),
        ),
        (
            0b11000_001_0000000,
            ("RDTIME", IF::I, EXT::Zicntr, 32),
        ),
        (
            0b11000_010_0000000,
            ("RDINSTRET", IF::I, EXT::Zicntr, 32),
        ),
        (
            0b11100_001_0000000,
            ("CSRRW", IF::I, EXT::Zicsr, 32),
        ),
        (
            0b11100_010_0000000,
            ("CSRRS", IF::I, EXT::Zicsr, 32),
        ),
        (
            0b11100_011_0000000,
            ("CSRRC", IF::I, EXT::Zicsr, 32),
        ),
        (
            0b00011_000_0000000,
            ("FENCE", IF::I, EXT::Zifencei, 32),
        ),
        (
            0b00011_000_1000001,
            ("FENCE.TSO", IF::I, EXT::Zifencei, 32),
        ),
        (
            0b11000_011_0000000,
            ("RDHPMCOUNTER", IF::I, EXT::Zihpm, 32),
        ),
    ])
});

pub fn get_mnemonic(
    opcode: Opcode,
    funct3: Option<Funct3>,
    funct7: Option<Funct7>,
) -> Option<Mnemonic> {
    let op: u16 = (u8::from(opcode) & 0b1111100).into();
    let f3: u16 = funct3.map_or(0, |val| u16::from(u8::from(val)));
    let f7: u16 = funct7.map_or(0, |val| u16::from(u8::from(val)));
    let code: u16 = (op << 8) | (f3 << 7) | f7;

    INSTRUCTIONS.get(&code).map(|res| res.0.into())
}

// #[cfg(test)]
// mod test {
//
//     use super::*;
//
//     #[test]
//     fn test_get_mnemonic() {
//         assert_eq!("add", get_mnemonic());
//     }
//
// }
