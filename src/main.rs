#[macro_use]
extern crate lazy_static;

mod error;
mod model;
mod data;
mod config;

use model::Instruction;
use model::InstructionTrait;
use model::RInstruction;

fn main() {
    let bits: u32 = 0x00b574b3;
    let instr = Instruction::try_from(bits).unwrap();
    println!("Format {}", instr.get_format());
    println!("Opcode {}", instr.get_opcode());
    println!("Compressed {}", instr.is_compressed());

    let instr = RInstruction::try_from(bits).unwrap();
    println!("R: {}", instr);


}
