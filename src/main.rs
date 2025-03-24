#[macro_use]
mod config;
mod data;
mod disasm;
mod error;
mod instr;
mod model;
mod utils;

use std::io::Cursor;

use disasm::Disasm;
use instr::Instruction;
use instr::InstructionTrait;

fn main() {
    let bits: u32 = 0x00b574b3;
    let instr = Instruction::try_from(bits).unwrap();
    println!("Format {}", instr.get_format());
    println!("Opcode {}", instr.get_opcode());
    println!("Compressed {}", instr.is_compressed());
    println!("{}", instr);

    println!("{}", Instruction::try_from(0x1ea5af23).unwrap());
    println!("{}", Instruction::try_from(0x80040293).unwrap());
    println!("{}", Instruction::try_from(0x00a5d463).unwrap()); // blt a1, a0, 0x101c4
    println!("{}", Instruction::try_from(0x000ff537).unwrap()); // lui a0, 0xff
    println!("{}", Instruction::try_from(0x0ab0256f).unwrap()); // jal a0, 0x28aa

    let data = vec![0x33, 0x85, 0x62, 0x00]; // add a0, t0, t1
    let cursor = Cursor::new(data);
    let mut disasm = Disasm::new(cursor);
    let _ = disasm.print_all();
}
