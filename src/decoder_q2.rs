use crate::decoder::{RDindex, RS1index};
use crate::instructions::Instruction;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    SLLI,
    FLDSP,
    LWSP,
    FLWSP,
    MISC,
    FSDSP,
    SWSP,
    FSWSP,
}

fn get_opcode(instruction: u32) -> Result<OpCode, &'static str> {
    match instruction >> 13 {
        0b000 => Ok(OpCode::SLLI),
        0b001 => Ok(OpCode::FLDSP),
        0b010 => Ok(OpCode::LWSP),
        0b011 => Ok(OpCode::FLWSP),
        0b100 => Ok(OpCode::MISC),
        0b101 => Ok(OpCode::FSDSP),
        0b110 => Ok(OpCode::SWSP),
        0b111 => Ok(OpCode::FSWSP),
        _ => Err("Invalid q2 opcode"),
    }
}

fn bit_from_to(inst: u32, from: u32, to: u32) -> u32 {
    ((inst >> from) & 1) << to
}

fn get_rd(inst: u32) -> RDindex {
    ((inst >> 7) & 0b1_1111) as RDindex
}

fn get_rs(inst: u32) -> RS1index {
    ((inst >> 2) & 0b1_1111) as RS1index
}

pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    let op = get_opcode(instruction)?;
    match op {
        OpCode::SLLI => {
            let rdindex = get_rd(instruction);
            let imm = 0;
            Ok(Instruction::CSLLI(rdindex, imm))
        }
        OpCode::FLDSP => {
            let rdindex = get_rd(instruction);
            let imm = 0;
            Ok(Instruction::CFLDSP(rdindex, imm))
        }
        OpCode::LWSP => {
            let rdindex = get_rd(instruction);
            let imm = 0;
            Ok(Instruction::CLWSP(rdindex, imm))
        }
        OpCode::FLWSP => {
            let rdindex = get_rd(instruction);
            let imm = 0;
            Ok(Instruction::CFLWSP(rdindex, imm))
        }
        OpCode::MISC => {
            let rdindex = get_rd(instruction);
            let rsindex = get_rs(instruction);
            let opt = ((instruction >> 12) & 1) == 1;
            if !opt && rsindex == 0 {
                Ok(Instruction::CJR(rdindex))
            } else if !opt && rsindex > 0 {
                Ok(Instruction::CMV(rdindex, rsindex))
            } else if opt && rsindex == 0 && rdindex == 0 {
                Ok(Instruction::CEBREAK())
            } else if opt && rsindex == 0 && rdindex != 0 {
                Ok(Instruction::CJALR(rdindex))
            } else {
                Ok(Instruction::CADD(rdindex, rsindex))
            }
        }
        OpCode::FSDSP => {
            let rsindex = get_rs(instruction);
            let imm = 0;
            Ok(Instruction::CFSDSP(rsindex, imm))
        }
        OpCode::SWSP => {
            let rsindex = get_rs(instruction);
            let imm = 0;
            Ok(Instruction::CSWSP(rsindex, imm))
        }
        OpCode::FSWSP => {
            let rsindex = get_rs(instruction);
            let imm = 0;
            Ok(Instruction::CFSWSP(rsindex, imm))
        }
    }
}
