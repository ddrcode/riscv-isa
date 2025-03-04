mod error;
mod format;
mod opcode;
mod instrtrait;
mod instruction;
mod rtype;
mod stype;
mod register;
mod immediate;
mod funct;
mod extension;


use format::InstructionFormat;
use instruction::{ Instruction };
use instrtrait::InstructionTrait;
use rtype::RInstruction;

fn main() {
    let bits: u32 = 0x00b574b3;
    let instr = Instruction::try_from(bits).unwrap();
    println!("Format {}", instr.get_format());
    println!("Opcode {}", instr.get_opcode());
    println!("Compressed {}", instr.is_compressed());


    let instr = RInstruction::try_from(bits).unwrap();
    println!("R: {}", instr);
}
