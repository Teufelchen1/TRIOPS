use crate::decoder::{CNZUimmediate, CUimmediate, RDindex, RS1index};
use crate::instructions::Instruction;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    ADDI4SPN,
    FLD,
    LW,
    FLW,
    RESERVED,
    FSD,
    SW,
    FSW,
}

fn get_rd(inst: u32) -> RDindex {
    (((inst >> 2) & 0b111) + 8) as RDindex
}

fn get_rs(inst: u32) -> RS1index {
    (((inst >> 7) & 0b111) + 8) as RS1index
}

fn get_imm(inst: u32) -> CUimmediate {
    (((inst >> 10) & 0b111) << 3) + (((inst >> 5) & 0b1) << 6) + (((inst >> 6) & 0b1) << 2)
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
        OpCode::FLD => Ok(Instruction::CFLD(rdindex, rs1index, imm)),
        OpCode::LW => Ok(Instruction::CLW(rdindex, rs1index, imm)),
        OpCode::FLW => Ok(Instruction::CFLW(rdindex, rs1index, imm)),
        OpCode::RESERVED => Err("Reserved instruction"),
        OpCode::FSD => Ok(Instruction::CFSD(rdindex, rs1index, imm)),
        OpCode::SW => Ok(Instruction::CSW(rs1index, rdindex, imm)),
        OpCode::FSW => Ok(Instruction::CFSW(rdindex, rs1index, imm)),
        OpCode::ADDI4SPN => {
            let rdindex = get_rd(instruction);
            let imm = ((((instruction >> 5) & 1) << 3)
                + (((instruction >> 6) & 1) << 2)
                + (((instruction >> 7) & 1) << 6)
                + (((instruction >> 8) & 1) << 7)
                + (((instruction >> 9) & 1) << 8)
                + (((instruction >> 10) & 1) << 9)
                + (((instruction >> 11) & 1) << 4)
                + (((instruction >> 12) & 1) << 5));
            Ok(Instruction::CADDI4SPN(rdindex, imm))
        }
    }
}
