use crate::{
    error::RISCVError,
    instr::{BInstruction, IInstruction, JInstruction, RInstruction, SInstruction, UInstruction},
    model::{Funct3, Funct7, Immediate, InstructionFormat, Opcode, Register},
};

use super::Instruction;

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
    pub fn set_immediate(mut self, val: i32) -> InstructionBuilder {
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

    pub fn build(self) -> Result<Instruction, RISCVError> {
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
