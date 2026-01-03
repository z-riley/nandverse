extern crate nandverse;

use nandverse::bus::{bus_to_num, to_bus};

const XLEN: usize = 32;

fn main() {
    println!("hi");
}

/// Pseudocode notation:
/// pc: program counter
/// rd: integer register destination
/// rsN: integer register source N
/// imm: immediate operand value
/// offset: immediate program counter relative offset
/// ux(reg): unsigned XLEN-bit integer (32-bit on RV32, 64-bit on RV64)
/// sx(reg): signed XLEN-bit integer (32-bit on RV32, 64-bit on RV64)
/// uN(reg): zero extended N-bit integer register value
/// sN(reg): sign extended N-bit integer register value
/// uN[reg + imm]: unsigned N-bit memory reference
/// sN[reg + imm]: signed N-bit memory reference
/// https://www.cs.unh.edu/~pjh/courses/cs520/15spr/riscv-rv32i-instructions.pdf
pub enum BaseInstruction {
    ///  Load Upper Immediate: rd ← imm
    LUI,
    ///  Add Upper Immediate to PC: rd ← pc + offset
    AUIPC,
    ///  Jump and Link: rd ← pc + length(inst) pc ← pc + offset
    JAL,
    ///  Jump and Link Register: rd ← pc + length(inst) pc ← (rs1 + offset) ∧ -2
    JALR,
    ///  Branch Equal: if rs1  = rs2  then pc ← pc + offset
    BEQ,
    ///  Branch Not Equal: if rs1  ≠ rs2  then pc ← pc + offset
    BNE,
    ///  Branch Less Than: if rs1  < rs2  then pc ← pc + offset
    BLT,
    ///  Branch Greater than Equal: if rs1  ≥ rs2  then pc ← pc + offset
    BGE,
    ///  Branch Less Than Unsigned: if rs1  < rs2  then pc ← pc + offset
    BLTU,
    ///  Branch Greater than Equal Unsigned: if rs1  ≥ rs2  then pc ← pc + offset
    BGEU,
    ///  Load Byte: rd ← s8[rs1   + offset]
    LB,
    ///  Load Half: rd ← s16[rs1   + offset]
    LH,
    ///  Load Word : rd ← s32[rs1   + offset]
    LW,
    ///  Load Byte Unsigned: rd ← u8[rs1   + offset]
    LBU,
    ///  Load Half Unsigned: rd ← u16[rs1  + offset]
    LHU,
    ///  Store Byte: u8[rs1 + offset] ← rs2
    SB,
    ///  Store Half: u16[rs1 + offset] ← rs2
    SH,
    ///  Store Word: [rs1  + offset] ← rs2
    SW,
    ///  Add Immediate: rd ← rs1  + sx(imm)
    ADDI,
    ///  Set Less Than Immediate: rd ← sx(rs1) < sx(imm)
    SLTI,
    ///  Set Less Than Immediate Unsigned: rd ← ux(rs1) < ux(imm)
    SLTIU,
    ///  Xor Immediate: rd ← ux(rs1) ⊕ ux(imm)
    XORI,
    ///  Or Immediate: rd ← ux(rs1) ∨ ux(imm)
    ORI,
    ///  And Immediate: rd ← ux(rs1) ∧ ux(imm)
    ANDI,
    ///  Shift Left Logical Immediate: rd ← ux(rs1) « ux(imm)
    SLLI,
    ///  Shift Right Logical Immediate: rd ← ux(rs1) » ux(imm)
    SRLI,
    ///  Shift Right Arithmetic Immediate: rd ← sx(rs1) » ux(imm)
    SRAI,
    ///  Add: rd ← sx(rs1) + sx(rs2)
    ADD,
    ///  Subtract: rd ← sx(rs1) - sx(rs2)
    SUB,
    ///  Shift Left Logical: rd ← ux(rs1) « rs2
    SLL,
    ///  Set Less Than: rd ← sx(rs1) < sx(rs2)
    SLT,
    ///  Set Less Than Unsigned: rd ← ux(rs1) < ux(rs2)
    SLTU,
    ///  Xor: rd ← ux(rs1) ⊕ ux(rs2)
    XOR,
    ///  Shift Right Logical: rd ← ux(rs1) » rs2
    SRL,
    ///  Shift Right Arithmetic: rd ← sx(rs1)  » rs2
    SRA,
    ///  Or: rd ← ux(rs1) ∨ ux(rs2)
    OR,
    ///  And: rd ← ux(rs1) ∧ ux(rs2)
    AND,
    /// Fence
    FENCE,
    /// Fence Instruction
    FENCEI,
}

impl BaseInstruction {
    pub fn value(&self) -> [bool; 6] {
        match self {
            // Upper immediate
            BaseInstruction::LUI => to_bus(0b0110111),
            BaseInstruction::AUIPC => to_bus(0b0010111),

            // Jump
            BaseInstruction::JAL => to_bus(0b1101111),

            // Immediate
            BaseInstruction::JALR => to_bus(0b1100111),

            // Branch
            BaseInstruction::BEQ => to_bus(0b1100011),
            BaseInstruction::BNE => to_bus(0b1100011),
            BaseInstruction::BLT => to_bus(0b1100011),
            BaseInstruction::BGE => to_bus(0b1100011),
            BaseInstruction::BLTU => to_bus(0b1100011),
            BaseInstruction::BGEU => to_bus(0b1100011),

            // Immediate
            BaseInstruction::LB => to_bus(0b0000011),
            BaseInstruction::LH => to_bus(0b0000011),
            BaseInstruction::LW => to_bus(0b0000011),
            BaseInstruction::LBU => to_bus(0b0000011),
            BaseInstruction::LHU => to_bus(0b0000011),

            // Store
            BaseInstruction::SB => to_bus(0b0100011),
            BaseInstruction::SH => to_bus(0b0100011),
            BaseInstruction::SW => to_bus(0b0100011),

            // Immediate
            BaseInstruction::ADDI => to_bus(0b0010011),
            BaseInstruction::SLTI => to_bus(0b0010011),
            BaseInstruction::SLTIU => to_bus(0b0010011),
            BaseInstruction::XORI => to_bus(0b0010011),
            BaseInstruction::ORI => to_bus(0b0010011),
            BaseInstruction::ANDI => to_bus(0b0010011),
            BaseInstruction::SLLI => to_bus(0b0010011),
            BaseInstruction::SRLI => to_bus(0b0010011),
            BaseInstruction::SRAI => to_bus(0b0010011),

            // Register/register
            BaseInstruction::ADD => to_bus(0b0110011),
            BaseInstruction::SUB => to_bus(0b0110011),
            BaseInstruction::SLL => to_bus(0b0110011),
            BaseInstruction::SLT => to_bus(0b0110011),
            BaseInstruction::SLTU => to_bus(0b0110011),
            BaseInstruction::XOR => to_bus(0b0110011),
            BaseInstruction::SRL => to_bus(0b0110011),
            BaseInstruction::SRA => to_bus(0b0110011),
            BaseInstruction::OR => to_bus(0b0110011),
            BaseInstruction::AND => to_bus(0b0110011),

            BaseInstruction::FENCE => to_bus(0b0001111),
            BaseInstruction::FENCEI => to_bus(0b0001111),
            // BaseInstruction::SCALL => to_bus(0b1110011),
            // BaseInstruction::SBREAK => to_bus(0b1110011),
            // BaseInstruction::RDCYCLE => to_bus(0b1110011),
            // BaseInstruction::RDCYCLEH => to_bus(0b1110011),
            // BaseInstruction::RDTIME => to_bus(0b1110011),
            // BaseInstruction::RDTIMEH => to_bus(0b1110011),
            // BaseInstruction::RDINSTRET => to_bus(0b1110011),
            // BaseInstruction::RDINSTRETH => to_bus(0b1110011),
        }
    }

    pub fn instruction_format(&self) -> InstructionFormat {
        match self {
            // Upper immediate
            BaseInstruction::LUI => InstructionFormat::UType,
            BaseInstruction::AUIPC => InstructionFormat::UType,

            // Jump
            BaseInstruction::JAL => InstructionFormat::JType,

            // Immediate
            BaseInstruction::JALR => InstructionFormat::IType,

            // Branch
            BaseInstruction::BEQ => InstructionFormat::BType,
            BaseInstruction::BNE => InstructionFormat::BType,
            BaseInstruction::BLT => InstructionFormat::BType,
            BaseInstruction::BGE => InstructionFormat::BType,
            BaseInstruction::BLTU => InstructionFormat::BType,
            BaseInstruction::BGEU => InstructionFormat::BType,

            // Immediate
            BaseInstruction::LB => InstructionFormat::IType,
            BaseInstruction::LH => InstructionFormat::IType,
            BaseInstruction::LW => InstructionFormat::IType,
            BaseInstruction::LBU => InstructionFormat::IType,
            BaseInstruction::LHU => InstructionFormat::IType,

            // Store
            BaseInstruction::SB => InstructionFormat::SType,
            BaseInstruction::SH => InstructionFormat::SType,
            BaseInstruction::SW => InstructionFormat::SType,

            // Immediate
            BaseInstruction::ADDI => InstructionFormat::IType,
            BaseInstruction::SLTI => InstructionFormat::IType,
            BaseInstruction::SLTIU => InstructionFormat::IType,
            BaseInstruction::XORI => InstructionFormat::IType,
            BaseInstruction::ORI => InstructionFormat::IType,
            BaseInstruction::ANDI => InstructionFormat::IType,
            BaseInstruction::SLLI => InstructionFormat::IType,
            BaseInstruction::SRLI => InstructionFormat::IType,
            BaseInstruction::SRAI => InstructionFormat::IType,

            // Register/register
            BaseInstruction::ADD => InstructionFormat::RType,
            BaseInstruction::SUB => InstructionFormat::RType,
            BaseInstruction::SLL => InstructionFormat::RType,
            BaseInstruction::SLT => InstructionFormat::RType,
            BaseInstruction::SLTU => InstructionFormat::RType,
            BaseInstruction::XOR => InstructionFormat::RType,
            BaseInstruction::SRL => InstructionFormat::RType,
            BaseInstruction::SRA => InstructionFormat::RType,
            BaseInstruction::OR => InstructionFormat::RType,
            BaseInstruction::AND => InstructionFormat::RType,

            BaseInstruction::FENCE => InstructionFormat::IType,
            BaseInstruction::FENCEI => InstructionFormat::IType,
            // BaseInstruction::SCALL => InstructionFormat::IType,
            // BaseInstruction::SBREAK => InstructionFormat::IType,
            // BaseInstruction::RDCYCLE => InstructionFormat::IType,
            // BaseInstruction::RDCYCLEH => InstructionFormat::IType,
            // BaseInstruction::RDTIME => InstructionFormat::IType,
            // BaseInstruction::RDTIMEH => InstructionFormat::IType,
            // BaseInstruction::RDINSTRET => InstructionFormat::IType,
            // BaseInstruction::RDINSTRETH => InstructionFormat::IType,
        }
    }
}

impl TryFrom<&[bool; XLEN]> for BaseInstruction {
    type Error = ();

    fn try_from(value: &[bool; XLEN]) -> Result<Self, Self::Error> {
        let opcode: [bool; 7] = value[0..7].try_into().unwrap();

        let format = InstructionFormat::try_from(&opcode).unwrap();

        match format {
            InstructionFormat::RType => todo!(),
            InstructionFormat::IType => todo!(),
            InstructionFormat::SType => todo!(),
            InstructionFormat::BType => todo!(),
            InstructionFormat::UType => todo!(),
            InstructionFormat::JType => todo!(),
        }
    }
}

pub enum InstructionFormat {
    /// Register/register.
    RType,
    /// Immediate.
    IType,
    /// Store.
    SType,
    /// Branch.
    BType,
    /// Upper immediate.
    UType,
    /// Jump.
    JType,
}

impl TryFrom<&[bool; 7]> for InstructionFormat {
    type Error = ();

    fn try_from(value: &[bool; 7]) -> Result<Self, Self::Error> {
        let opcode = bus_to_num::<u8>(value);
        match opcode {
            0b0110011 => Ok(InstructionFormat::RType),

            0b0010011 => Ok(InstructionFormat::IType),
            0b0000011 => Ok(InstructionFormat::IType),
            0b1100111 => Ok(InstructionFormat::IType),
            0b1110011 => Ok(InstructionFormat::IType),

            0b0100011 => Ok(InstructionFormat::SType),

            0b1100011 => Ok(InstructionFormat::BType),

            0b0110111 => Ok(InstructionFormat::UType),
            0b0010111 => Ok(InstructionFormat::UType),

            0b1101111 => Ok(InstructionFormat::JType),
            _ => Err(()),
        }
    }
}

pub struct Cpu {
    /// Register file.
    rf: RegisterFile,
    /// ALU.
    alu: Alu,
}

pub struct RegisterFile {
    // Hardwired to contant 0.
    x0: Register,
    /// General purpose registers: x0 to xXLEN. Details at https://en.wikichip.org/wiki/risc-v/registers.
    general: [Register; XLEN],
    /// Program counter. Cannot be written or read using load/store instructions.
    pc: Register,
}

struct Register {}

pub struct Alu {}

impl Alu {
    pub fn execute_instruction(instruction: &[bool; XLEN]) {
        // 1. Decode opcode
        // 2. If required, decode funct3
        // 3. If required, decode funct7
        // 4. Execute instruction
    }

    fn decode_opcode(instruction: &[bool; XLEN]) -> BaseInstruction {
        let _instruction = BaseInstruction::try_from(instruction).unwrap();
        todo!()
    }
}

pub fn alu(op: &[bool; 6], a: &[bool; XLEN], b: &[bool; XLEN]) -> [bool; XLEN] {
    todo!()
}
