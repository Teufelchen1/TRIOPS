use crate::instructions::{bit_from_to, Immediate, Instruction, RDindex, RS1index};

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

fn get_rd(inst: u32) -> RDindex {
    ((inst >> 7) & 0b1_1111) as RDindex
}

fn get_rs(inst: u32) -> RS1index {
    ((inst >> 2) & 0b1_1111) as RS1index
}

fn get_ci_offset(inst: u32) -> Immediate {
    (bit_from_to(inst, 2, 6)
        + bit_from_to(inst, 3, 7)
        + bit_from_to(inst, 4, 2)
        + bit_from_to(inst, 5, 3)
        + bit_from_to(inst, 6, 4)
        + bit_from_to(inst, 12, 5)) as Immediate
}

fn get_css_offset(inst: u32) -> Immediate {
    (bit_from_to(inst, 7, 6)
        + bit_from_to(inst, 8, 7)
        + bit_from_to(inst, 9, 2)
        + bit_from_to(inst, 10, 3)
        + bit_from_to(inst, 11, 4)
        + bit_from_to(inst, 12, 5)) as Immediate
}

fn get_shamt(inst: u32) -> i32 {
    (((inst >> 2) & 0b1_1111) + bit_from_to(inst, 12, 5)) as i32
}

pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    let op = get_opcode(instruction)?;
    match op {
        OpCode::SLLI => {
            let rdindex = get_rd(instruction);
            let imm = get_shamt(instruction);
            Ok(Instruction::CSLLI(rdindex, imm))
        }
        OpCode::FLDSP => Err("FLDSP not implemented"),
        OpCode::LWSP => {
            let rdindex = get_rd(instruction);
            let imm = get_ci_offset(instruction);
            Ok(Instruction::CLWSP(rdindex, imm))
        }
        OpCode::FLWSP => Err("FLWSP not implemented"),
        OpCode::MISC => {
            let rdindex = get_rd(instruction);
            let rs1index = get_rs(instruction);
            let opt = ((instruction >> 12) & 1) == 1;
            if !opt && rs1index == 0 {
                Ok(Instruction::CJR(rdindex))
            } else if !opt && rs1index > 0 {
                Ok(Instruction::CMV(rdindex, rs1index))
            } else if opt && rs1index == 0 && rdindex == 0 {
                Ok(Instruction::CEBREAK())
            } else if opt && rs1index == 0 && rdindex != 0 {
                Ok(Instruction::CJALR(rdindex))
            } else {
                Ok(Instruction::CADD(rdindex, rs1index))
            }
        }
        OpCode::FSDSP => Err("FSDSP not implemented"),
        OpCode::SWSP => {
            let rsindex = get_rs(instruction);
            let imm = get_css_offset(instruction);
            Ok(Instruction::CSWSP(rsindex, imm))
        }
        OpCode::FSWSP => Err("FSWSP not implemented"),
    }
}
