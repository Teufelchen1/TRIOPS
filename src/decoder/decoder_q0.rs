use crate::decoder::{bit_from_to, Immediate, RDindex, RS1index};
use crate::instructions::Instruction;

#[derive(Debug, PartialEq)]
enum OpCode {
    ADDI4SPN,
    FLD,
    LW,
    FLW,
    RESERVED,
    FSD,
    SW,
    FSW,
}

/* Valid for CIW, CL and CS */
fn get_rd(inst: u32) -> RDindex {
    /* Add 8 converts compressed register to actual register number */
    (((inst >> 2) & 0b111) + 8) as RDindex
}

/* Valid for CL and CS */
fn get_rs(inst: u32) -> RS1index {
    /* Add 8 converts compressed register to actual register number */
    (((inst >> 7) & 0b111) + 8) as RS1index
}

/* Valid for CL and CS */
fn get_imm(inst: u32) -> Immediate {
    ((((inst >> 10) & 0b111) << 3) + bit_from_to(inst, 5, 6) + bit_from_to(inst, 6, 2)) as i32
}

fn get_opcode(instruction: u32) -> Result<OpCode, &'static str> {
    match instruction >> 13 {
        0b000 => Ok(OpCode::ADDI4SPN),
        0b001 => Ok(OpCode::FLD),
        0b010 => Ok(OpCode::LW),
        0b011 => Ok(OpCode::FLW),
        0b100 => Ok(OpCode::RESERVED),
        0b101 => Ok(OpCode::FSD),
        0b110 => Ok(OpCode::SW),
        0b111 => Ok(OpCode::FSW),
        _ => Err("Invalid q0 opcode"),
    }
}

pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    if instruction == 0 {
        return Err("Instruction is zero");
    }
    let op = get_opcode(instruction)?;
    let rdindex = get_rd(instruction);
    let rs1index = get_rs(instruction);
    let imm = get_imm(instruction);
    match op {
        OpCode::FLD => Err("C.FLD not implemented"),
        OpCode::LW => Ok(Instruction::CLW(rdindex, rs1index, imm)),
        OpCode::FLW => Err("C.FLW not implemented"),
        OpCode::RESERVED => Err("Reserved instruction"),
        OpCode::FSD => Err("C.FSD not implemented"),
        OpCode::SW => Ok(Instruction::CSW(rs1index, rdindex, imm)),
        OpCode::FSW => Err("C.FSW not implemented"),
        OpCode::ADDI4SPN => {
            let imm = (bit_from_to(instruction, 5, 3)
                + bit_from_to(instruction, 6, 2)
                + bit_from_to(instruction, 7, 6)
                + bit_from_to(instruction, 8, 7)
                + bit_from_to(instruction, 9, 8)
                + bit_from_to(instruction, 10, 9)
                + bit_from_to(instruction, 11, 4)
                + bit_from_to(instruction, 12, 5)) as Immediate;
            Ok(Instruction::CADDI4SPN(rdindex, imm))
        }
    }
}
