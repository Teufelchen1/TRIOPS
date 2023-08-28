use crate::decoder::{RDindex, RS1index};
use crate::instructions::Instruction;

#[derive(Debug, PartialEq)]
pub enum OpCode {
    ADDI,
    JAL,
    LI,
    LUI,
    ALU,
    J,
    BEQZ,
    BNEZ,
}

fn sign_extend(num: u32, bitnum: u32) -> u32 {
    let msb = num >> (bitnum - 1);
    let sign_filled = {
        if msb == 0 {
            0x0
        } else {
            (!0x0u32).checked_shl(bitnum).unwrap_or(0)
        }
    };
    sign_filled | num
}

fn bit_from_to(inst: u32, from: u32, to: u32) -> u32 {
    ((inst >> from) & 1) << to
}

fn get_imm(inst: u32) -> u32 {
    ((inst >> 2) & 0b1_1111) + bit_from_to(inst, 12, 5)
}

fn get_jimm(inst: u32) -> u32 {
    bit_from_to(inst, 2, 5)
        + bit_from_to(inst, 3, 1)
        + bit_from_to(inst, 4, 2)
        + bit_from_to(inst, 5, 3)
        + bit_from_to(inst, 6, 7)
        + bit_from_to(inst, 7, 6)
        + bit_from_to(inst, 8, 10)
        + bit_from_to(inst, 9, 8)
        + bit_from_to(inst, 10, 9)
        + bit_from_to(inst, 11, 4)
        + bit_from_to(inst, 12, 11)
}

fn get_bimm(inst: u32) -> u32 {
    bit_from_to(inst, 2, 5)
        + bit_from_to(inst, 3, 1)
        + bit_from_to(inst, 4, 2)
        + bit_from_to(inst, 5, 6)
        + bit_from_to(inst, 6, 7)
        + bit_from_to(inst, 10, 3)
        + bit_from_to(inst, 11, 4)
        + bit_from_to(inst, 12, 8)
}

fn get_rs(inst: u32) -> RS1index {
    (((inst >> 7) & 0b111) + 8) as RS1index
}

fn get_opcode(instruction: u32) -> Result<OpCode, &'static str> {
    match instruction >> 13 {
        0b000 => Ok(OpCode::ADDI),
        0b001 => Ok(OpCode::JAL),
        0b010 => Ok(OpCode::LI),
        0b011 => Ok(OpCode::LUI),
        0b100 => Ok(OpCode::ALU),
        0b101 => Ok(OpCode::J),
        0b110 => Ok(OpCode::BEQZ),
        0b111 => Ok(OpCode::BNEZ),
        _ => Err("Invalid q1 opcode"),
    }
}

pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    let op = get_opcode(instruction)?;
    match op {
        OpCode::ADDI => {
            if instruction == 0 {
                return Ok(Instruction::CNOP(0, 0));
            }
            let rdindex = ((instruction >> 7) & 0b1_1111) as RDindex;
            let imm = get_imm(instruction);
            Ok(Instruction::CADDI(rdindex, imm))
        }
        OpCode::JAL => {
            let imm = get_jimm(instruction);
            Ok(Instruction::CJAL(imm))
        }
        OpCode::LI => {
            let rdindex = ((instruction >> 7) & 0b1_1111) as RDindex;
            let imm = sign_extend(get_imm(instruction), 6);
            Ok(Instruction::CLI(rdindex, imm))
        }
        OpCode::LUI => {
            let rdindex = ((instruction >> 7) & 0b1_1111) as RDindex;
            let imm = get_imm(instruction);
            assert!(rdindex != 0);
            if rdindex == 2 {
                Ok(Instruction::CADDI16SP(2, imm))
            } else {
                Ok(Instruction::CLUI(rdindex, imm))
            }
        }
        OpCode::ALU => {
            let opt1110 = (instruction >> 10) & 0b11;
            let opt56 = (instruction >> 5) & 0b11;
            let imm = get_imm(instruction);
            let rsindex = get_rs(instruction);
            match opt1110 {
                0b00 => Ok(Instruction::CSRLI(rsindex, imm)),
                0b01 => Ok(Instruction::CSRAI(rsindex, imm)),
                0b10 => Ok(Instruction::CANDI(rsindex, imm)),
                0b11 => {
                    let rs2index = ((instruction >> 2) & 0b111) as RS1index;
                    match opt56 {
                        0b00 => Ok(Instruction::CSUB(rsindex, rs2index)),
                        0b01 => Ok(Instruction::CXOR(rsindex, rs2index)),
                        0b10 => Ok(Instruction::COR(rsindex, rs2index)),
                        0b11 => Ok(Instruction::CAND(rsindex, rs2index)),
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
        OpCode::J => {
            let imm = get_jimm(instruction);
            Ok(Instruction::CJ(imm))
        }
        OpCode::BEQZ => {
            let rsindex = get_rs(instruction);
            let imm = get_bimm(instruction);
            Ok(Instruction::CBEQZ(rsindex, imm))
        }
        OpCode::BNEZ => {
            let rsindex = get_rs(instruction);
            let imm = get_bimm(instruction);
            Ok(Instruction::CBNEZ(rsindex, imm))
        }
    }
}
