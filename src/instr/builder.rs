use crate::{
    error::RISCVError,
    instr::{BInstruction, IInstruction, JInstruction, RInstruction, SInstruction, UInstruction},
    model::{Funct3, Funct7, Immediate, InstructionFormat, Opcode, Register},
};

use super::{Instruction, InstructionTrait};

/// A builder for constructing RISC-V instructions.
///
/// The type of the output instruction is determined by the opcode.
///
/// Values not required by a specific instruction type will be ignored.
/// For example, if an immediate value is provided for an R-type instruction (which doesn't have an immediate),
/// it will not cause an error; the value will simply be ignored.
///
/// # Example
///
/// ```rust
/// use riscv_isa::instr::{InstructionBuilder, Instruction};
/// use riscv_isa::model::{Opcode, Register};
/// # use riscv_isa::error::RISCVError;
///
/// # fn create_add() -> Result<Instruction, RISCVError> {
/// let instruction = InstructionBuilder::new()
///     .set_opcode(Opcode::try_from(0b0110011u8)?)
///     .set_funct3(0b000.into())
///     .set_funct7(0b0000000.into())
///     .set_rd(Register::a0())
///     .set_rs1(Register::t0())
///     .set_rs2(Register::t1())
///     .build()?;
///
/// assert_eq!("ADD a0, t0, t1", instruction.to_string());
/// assert_eq!(0x00628533, u32::from(instruction));
///
/// # Ok(instruction)
/// # }
/// ```
#[derive(Default)]
pub struct InstructionBuilder {
    opcode: Option<Opcode>,
    funct3: Option<Funct3>,
    funct7: Option<Funct7>,
    rs1: Option<Register>,
    rs2: Option<Register>,
    rd: Option<Register>,
    immediate: Option<i32>,
}

impl InstructionBuilder {

    /// Creates a new `InstructionBuilder` with all parameters unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the opcode for the instruction.
    ///
    /// # Arguments
    ///
    /// * `opcode` - The opcode to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_opcode(&mut self, opcode: Opcode) -> &mut InstructionBuilder {
        self.opcode = Some(opcode);
        self
    }

    /// Sets the funct3 field for the instruction.
    ///
    /// # Arguments
    ///
    /// * `funct3` - The funct3 value to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_funct3(&mut self, funct3: Funct3) -> &mut InstructionBuilder {
        self.funct3 = Some(funct3);
        self
    }

    /// Sets the funct7 field for the instruction.
    ///
    /// # Arguments
    ///
    /// * `funct7` - The funct7 value to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_funct7(&mut self, funct7: Funct7) -> &mut InstructionBuilder {
        self.funct7 = Some(funct7);
        self
    }

    /// Sets the rs1 register for the instruction.
    ///
    /// # Arguments
    ///
    /// * `reg` - The rs1 register to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_rs1(&mut self, reg: Register) -> &mut InstructionBuilder {
        self.rs1 = Some(reg);
        self
    }

    /// Sets the rs2 register for the instruction.
    ///
    /// # Arguments
    ///
    /// * `reg` - The rs2 register to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_rs2(&mut self, reg: Register) -> &mut InstructionBuilder {
        self.rs2 = Some(reg);
        self
    }

    /// Sets the rd register for the instruction.
    ///
    /// # Arguments
    ///
    /// * `reg` - The rd register to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_rd(&mut self, reg: Register) -> &mut InstructionBuilder {
        self.rd = Some(reg);
        self
    }

    /// Sets the immediate value for the instruction.
    ///
    /// # Arguments
    ///
    /// * `val` - The immediate value to set.
    ///
    /// # Returns
    ///
    /// A mutable reference to the `InstructionBuilder`.
    pub fn set_immediate(&mut self, val: i32) -> &mut InstructionBuilder {
        self.immediate = Some(val);
        self
    }

    fn opcode(&self) -> Result<Opcode, RISCVError> {
        self.opcode
            .ok_or(RISCVError::BuilderError("Opcode not provided".to_string()))
    }

    fn rs1(&self) -> Result<Register, RISCVError> {
        self.rs1.ok_or(RISCVError::BuilderError(
            "rs1 value not provided".to_string(),
        ))
    }

    fn rs2(&self) -> Result<Register, RISCVError> {
        self.rs2.ok_or(RISCVError::BuilderError(
            "rs2 value not provided".to_string(),
        ))
    }

    fn rd(&self) -> Result<Register, RISCVError> {
        self.rd.ok_or(RISCVError::BuilderError(
            "rd value not provided".to_string(),
        ))
    }

    fn funct3(&self) -> Result<Funct3, RISCVError> {
        self.funct3.ok_or(RISCVError::BuilderError(
            "funct3 value not provided".to_string(),
        ))
    }

    fn funct7(&self) -> Result<Funct7, RISCVError> {
        self.funct7.ok_or(RISCVError::BuilderError(
            "funct7 value not provided".to_string(),
        ))
    }

    fn immediate(&self) -> Result<i32, RISCVError> {
        self.immediate.ok_or(RISCVError::BuilderError(
            "immediate value not provided".to_string(),
        ))
    }

    /// Resets the builder to its default state.
    pub fn reset(&mut self) {
        self.opcode = None;
        self.funct3 = None;
        self.funct7 = None;
        self.rs1 = None;
        self.rs2 = None;
        self.rd = None;
        self.immediate = None;
    }

    /// Builds the instruction based on the provided fields.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `Instruction` or a `RISCVError` if any required fields are missing.
    pub fn build(&self) -> Result<Instruction, RISCVError> {
        use InstructionFormat::*;

        let opcode = self.opcode()?;

        let instr = match opcode.format() {
            B => Instruction::B(BInstruction::new(
                opcode,
                self.rs1()?,
                self.rs2()?,
                self.funct3()?,
                Immediate::<1, 12>::try_from(self.immediate()?)?,
            )?),
            I => Instruction::I(IInstruction::new(
                opcode,
                self.rs1()?,
                self.rd()?,
                self.funct3()?,
                Immediate::<0, 11>::try_from(self.immediate()?)?,
            )?),
            J => Instruction::J(JInstruction::new(
                opcode,
                self.rd()?,
                Immediate::<1, 20>::try_from(self.immediate()?)?,
            )?),
            R => Instruction::R(RInstruction::new(
                opcode,
                self.rs1()?,
                self.rs2()?,
                self.rd()?,
                self.funct3()?,
                self.funct7()?,
            )?),
            S => Instruction::S(SInstruction::new(
                opcode,
                self.rs1()?,
                self.rs2()?,
                self.funct3()?,
                Immediate::<0, 11>::try_from(self.immediate()?)?,
            )?),
            U => Instruction::U(UInstruction::new(
                opcode,
                self.rd()?,
                Immediate::<12, 31>::try_from(self.immediate()?)?,
            )?),
        };

        Ok(instr)
    }
}

impl From<&Instruction> for InstructionBuilder {
    /// Creates an `InstructionBuilder` from an existing `Instruction`.
    ///
    /// # Arguments
    ///
    /// * `instr` - The instruction to convert.
    ///
    /// # Returns
    ///
    /// An `InstructionBuilder` initialized with the fields from the provided instruction.
    fn from(instr: &Instruction) -> Self {
        InstructionBuilder {
            opcode: Some(*instr.opcode()),
            funct3: instr.funct3(),
            funct7: instr.funct7(),
            rs1: instr.rs1(),
            rs2: instr.rs2(),
            rd: instr.rd(),
            immediate: instr.immediate()
        }
    }
}

impl From<Instruction> for InstructionBuilder {
    /// Creates an `InstructionBuilder` from an existing `Instruction`.
    ///
    /// # Arguments
    ///
    /// * `instr` - The instruction to convert.
    ///
    /// # Returns
    ///
    /// An `InstructionBuilder` initialized with the fields from the provided instruction.
    fn from(instr: Instruction) -> Self {
        InstructionBuilder::from(&instr)
    }
}
