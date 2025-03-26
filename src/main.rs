pub mod config;
pub mod data;
mod disasm;
mod error;
pub mod instr;
pub mod model;
pub mod utils;

pub use disasm::*;
pub use error::RISCVError;

use std::env;
use std::fs::File;
use std::io::{BufReader, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut config = DisasmConfig::default();
    config.mnemonic_uppercase = false;
    config.mnemonic_separator = "\t".to_string();
    config.register_separator = "\t".to_string();

    let mut disasm = Disasm::with_config(reader, config);
    if let Err(e) = disasm.print_all() {
        println!("Error disassembling instruction: {:?}", e);
    }

    Ok(())
}
