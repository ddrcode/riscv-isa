use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::model::{Funct3, Funct7, InstructionFormat as IF, Opcode, RISCVExtension as EXT};

type Row = (String, IF, EXT, u8);

lazy_static! {
    static ref INSTRUCTIONS: HashMap<u16, Row> = HashMap::from([
        (0b01011_010_0000000, ("LR.W".to_string(), IF::R, EXT::A, 32)),
        (0b01011_010_0000001, ("SC.W".to_string(), IF::R, EXT::A, 32)),
        (
            0b01011_011_0000000,
            ("AMOSWAP.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_0000001,
            ("AMOADD.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_0010000,
            ("AMOXOR.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_0110000,
            ("AMOAND.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_0100000,
            ("AMOOR.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_1000000,
            ("AMOMIN.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_1010000,
            ("AMOMAX.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_1100000,
            ("AMOMINU.W".to_string(), IF::R, EXT::A, 32)
        ),
        (
            0b01011_011_1110000,
            ("AMOMAXU.W".to_string(), IF::R, EXT::A, 32)
        ),
        (0b10100_001_0000000, ("FLD".to_string(), IF::I, EXT::D, 64)),
        (0b10100_011_0000000, ("FSD".to_string(), IF::S, EXT::D, 64)),
        (0b10100_000_0000000, ("FLW".to_string(), IF::I, EXT::F, 32)),
        (0b10100_010_0000000, ("FSW".to_string(), IF::S, EXT::F, 32)),
        (
            0b11110_000_0000000,
            ("HLV.W".to_string(), IF::I, EXT::H, 64)
        ),
        (
            0b11110_000_0000001,
            ("HSV.W".to_string(), IF::S, EXT::H, 64)
        ),
        (0b01101_000_0000000, ("LUI".to_string(), IF::U, EXT::I, 32)),
        (
            0b00101_000_0000000,
            ("AUIPC".to_string(), IF::U, EXT::I, 32)
        ),
        (0b11011_000_0000000, ("JAL".to_string(), IF::J, EXT::I, 32)),
        (0b01001_000_0000000, ("JALR".to_string(), IF::I, EXT::I, 32)),
        (0b01000_000_0000000, ("BEQ".to_string(), IF::B, EXT::I, 32)),
        (0b01000_001_0000000, ("BNE".to_string(), IF::B, EXT::I, 32)),
        (0b01000_100_0000000, ("BLT".to_string(), IF::B, EXT::I, 32)),
        (0b01000_101_0000000, ("BGE".to_string(), IF::B, EXT::I, 32)),
        (0b01000_110_0000000, ("BLTU".to_string(), IF::B, EXT::I, 32)),
        (0b01000_111_0000000, ("BGEU".to_string(), IF::B, EXT::I, 32)),
        (0b00000_000_0000000, ("LB".to_string(), IF::I, EXT::I, 32)),
        (0b00000_001_0000000, ("LH".to_string(), IF::I, EXT::I, 32)),
        (0b00000_010_0000000, ("LW".to_string(), IF::I, EXT::I, 32)),
        (0b00000_100_0000000, ("LBU".to_string(), IF::I, EXT::I, 32)),
        (0b00000_101_0000000, ("LHU".to_string(), IF::I, EXT::I, 32)),
        (0b01000_000_0000000, ("SB".to_string(), IF::S, EXT::I, 32)),
        (0b01000_001_0000000, ("SH".to_string(), IF::S, EXT::I, 32)),
        (0b01000_010_0000000, ("SW".to_string(), IF::S, EXT::I, 32)),
        (0b00100_000_0000000, ("ADDI".to_string(), IF::I, EXT::I, 32)),
        (0b00100_010_0000000, ("SLTI".to_string(), IF::I, EXT::I, 32)),
        (
            0b00100_011_0000000,
            ("SLTIU".to_string(), IF::I, EXT::I, 32)
        ),
        (0b00100_100_0000000, ("XORI".to_string(), IF::I, EXT::I, 32)),
        (0b00100_110_0000000, ("ORI".to_string(), IF::I, EXT::I, 32)),
        (0b00100_111_0000000, ("ANDI".to_string(), IF::I, EXT::I, 32)),
        (0b00100_001_0000000, ("SLLI".to_string(), IF::I, EXT::I, 32)),
        (0b00100_101_0000000, ("SRLI".to_string(), IF::I, EXT::I, 32)),
        (0b00100_101_0100000, ("SRAI".to_string(), IF::I, EXT::I, 32)),
        (0b01100_000_0000000, ("ADD".to_string(), IF::R, EXT::I, 32)),
        (0b01100_000_0100000, ("SUB".to_string(), IF::R, EXT::I, 32)),
        (0b01100_001_0000000, ("SLL".to_string(), IF::R, EXT::I, 32)),
        (0b01100_010_0000000, ("SLT".to_string(), IF::R, EXT::I, 32)),
        (0b01100_011_0000000, ("SLTU".to_string(), IF::R, EXT::I, 32)),
        (0b01100_100_0000000, ("XOR".to_string(), IF::R, EXT::I, 32)),
        (0b01100_101_0000000, ("SRL".to_string(), IF::R, EXT::I, 32)),
        (0b01100_101_0100000, ("SRA".to_string(), IF::R, EXT::I, 32)),
        (0b01100_110_0000000, ("OR".to_string(), IF::R, EXT::I, 32)),
        (0b01100_111_0000000, ("AND".to_string(), IF::R, EXT::I, 32)),
        (
            0b11100_000_0000000,
            ("ECALL".to_string(), IF::I, EXT::I, 32)
        ),
        (
            0b11100_000_0000001,
            ("EBREAK".to_string(), IF::I, EXT::I, 32)
        ),
        (0b01100_000_0000001, ("MUL".to_string(), IF::R, EXT::M, 32)),
        (0b01100_001_0000001, ("MULH".to_string(), IF::R, EXT::M, 32)),
        (
            0b01100_010_0000001,
            ("MULHSU".to_string(), IF::R, EXT::M, 32)
        ),
        (
            0b01100_011_0000001,
            ("MULHU".to_string(), IF::R, EXT::M, 32)
        ),
        (0b01100_100_0000001, ("DIV".to_string(), IF::R, EXT::M, 32)),
        (0b01100_101_0000001, ("DIVU".to_string(), IF::R, EXT::M, 32)),
        (0b01100_110_0000001, ("REM".to_string(), IF::R, EXT::M, 32)),
        (0b01100_111_0000001, ("REMU".to_string(), IF::R, EXT::M, 32)),
        (0b10100_011_0000001, ("FLQ".to_string(), IF::I, EXT::Q, 64)),
        (0b10100_011_0000011, ("FSQ".to_string(), IF::S, EXT::Q, 64)),
        (
            0b11110_000_0000010,
            ("SSTC.SET".to_string(), IF::I, EXT::Sstc, 64)
        ),
        (
            0b11101_000_0000000,
            ("VADDR.TRANS".to_string(), IF::I, EXT::Svadu, 64)
        ),
        (
            0b00000_000_0001000,
            ("PAUSE.W".to_string(), IF::I, EXT::Zawrs, 32)
        ),
        (
            0b00010_001_0000000,
            ("CBO.FLUSH".to_string(), IF::I, EXT::Zicbom, 32)
        ),
        (
            0b00010_000_0000000,
            ("CBO.ZERO".to_string(), IF::I, EXT::Zicboz, 32)
        ),
        (
            0b11000_000_0000000,
            ("RDCYCLE".to_string(), IF::I, EXT::Zicntr, 32)
        ),
        (
            0b11000_001_0000000,
            ("RDTIME".to_string(), IF::I, EXT::Zicntr, 32)
        ),
        (
            0b11000_010_0000000,
            ("RDINSTRET".to_string(), IF::I, EXT::Zicntr, 32)
        ),
        (
            0b11100_001_0000000,
            ("CSRRW".to_string(), IF::I, EXT::Zicsr, 32)
        ),
        (
            0b11100_010_0000000,
            ("CSRRS".to_string(), IF::I, EXT::Zicsr, 32)
        ),
        (
            0b11100_011_0000000,
            ("CSRRC".to_string(), IF::I, EXT::Zicsr, 32)
        ),
        (
            0b00011_000_0000000,
            ("FENCE".to_string(), IF::I, EXT::Zifencei, 32)
        ),
        (
            0b00011_000_1000001,
            ("FENCE.TSO".to_string(), IF::I, EXT::Zifencei, 32)
        ),
        (
            0b11000_011_0000000,
            ("RDHPMCOUNTER".to_string(), IF::I, EXT::Zihpm, 32)
        ),
    ]);
}

pub fn get_mnemonic(
    opcode: Opcode,
    funct3: Option<Funct3>,
    funct7: Option<Funct7>,
) -> Option<&'static str> {
    let op: u16 = (u8::from(opcode) & 0b1111100).into();
    let f3: u16 = funct3.map_or(0, |val| u16::from(u8::from(val)));
    let f7: u16 = funct7.map_or(0, |val| u16::from(u8::from(val)));
    let code: u16 = (op << 8) | (f3 << 7) | f7;

    INSTRUCTIONS.get(&code).map(|res| res.0.as_str())
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
