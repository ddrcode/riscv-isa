# RISC-V ISA (Rust library)

A Rust library for representing the RISC-V Instruction Set Architecture (ISA) and for disassembling RISC-V binary code. This project aims to provide a rock‑solid, type‑safe model for RISC-V instructions, along with a disassembler that leverages this model to produce human‑readable assembly.

**Note** - the library is still under heavy development, and the API can change from version to version, providing breaking changes.

## Features

- **RISC-V ISA Representation**
  - Strongly typed models for various instruction components: opcodes, funct3, funct7, immediate values, and registers.
  - Use of Rust’s advanced type system to ensure that only valid values can be constructed.

- **Disassembler**
  - An iterator-based disassembler that reads a binary source and produces structured instruction representations.
  - Extensible design to support custom instructions and extensions.
  - Error handling via custom error types that wrap both I/O and RISC-V–specific errors.

- **Idiomatic Rust Design**
  - Minimal runtime overhead with extensive compile-time checks.

## Current limitations

The current version of the library has some limitations, that are planned to be addressed
in the future versions:

- support for compressed instruction (16-bit instructions),
- support for custom instructions of size different from 32-bits,
- assembly code parsing

## Examples of use

The code below demonstrates how to use the library to build a simple CLI tool
that takes RISC-V binary as an input and prints disassembled code to the console.


```Rust
use std::env;
use std::fs::File;
use std::io::{BufReader, Result};
use riscv_isa::Disasm;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let disasm = Disasm::new(reader);
    for result in disasm {
        match result {
            Ok(record) => println!("{}", record),
            Err(e) => {
                eprintln!("Error disassembling instruction: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
```

The above example could be simplified even further, by skipping the loop entirely:

``` Rust
    let mut disasm = Disasm::new(reader);
    if let Err(e) = disasm.print_all() {
        eprintln!("Error disassembling instruction: {:?}", e);
    }
```

A developer has a full control over how the disassembled code is formatted:

```Rust
    let mut config = DisasmConfig::default();
    config.mnemonic_uppercase = false;
    config.register_separator = "\t".to_string();
    config.immediate_format = |imm: i32| format!("{:08x}", imm);

    let mut disasm = Disasm::with_config(reader, config);
    disasm.print_all()?;
```

Besides disassembly, the library can be also used to build instructions from scratch
and produce instruction binary (so it's possible to build an assembler with it).
The code below creates an I-type instruction (`LBU s1, 0xff(a0)`):

```Rust
    let opcode = Opcode::try_from(0b0000011u32)?;
    let rs1 = Register::a0();
    let rd = Register::try_from(9)?; // s1
    let funct3 = Funct3::try_from(0b100u8)?;
    let imm = Immediate::<0, 11>::try_from(0xff)?;
    let instr = IInstruction::new(opcode, rs1, rd, funct3, imm)?;

    let instr_binary = u32::from(instr);
    let instr_bytes = instr_binary.to_le_bytes();
```

The above example can be written much more elegantly with `InstructionBuilder`:

```Rust
    let instr = InstructionBuilder::new()
        .set_opcode(Opcode::try_from(0b0000011u32)?)
        .set_rs1(Register::a0())
        .set_rd(Register::s1())
        .set_funct3(Funct3::try_from(0b100u8)?)
        .set_immediate(0xff)
        .build()?;
```

An individual instruction structure can be created directly from instruction binary,
without calling disassembler:

```Rust
    let instr = Instruction::try_from(0x00a5d463)?;
    assert_eq!("B", instr.format().to_string());
    assert_eq!("BLT a1, a0, 0x101c4", instr.to_string());
```

## License

Licensed under MIT license
   ([LICENSE](LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
