#[macro_use]
extern crate lazy_static;

mod error;
mod model;
mod data;
mod config;
mod instr;

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
}
