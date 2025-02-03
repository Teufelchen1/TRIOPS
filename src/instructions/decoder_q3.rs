use crate::instructions::{sign_extend, Immediate, Instruction, RDindex, RS1index, RS2index};

type Funct3 = u32;
type Funct5 = u32;
type Funct7 = u32;

fn immediate_i(instruction: u32) -> Immediate {
    sign_extend(instruction >> 20, 12) as Immediate
}

fn immediate_system(instruction: u32) -> u32 {
    instruction >> 20
}

fn immediate_s(instruction: u32) -> Immediate {
    sign_extend(
        ((instruction >> 25) << 5) | ((instruction >> 7) & 0b1_1111),
        12,
    ) as Immediate
}

fn immediate_b(instruction: u32) -> Immediate {
    let bits_4_1 = (instruction >> 8) & 0b1111;
    let bits_10_5 = (instruction >> 25) & 0b11_1111;
    let bits_11 = (instruction >> 7) & 0b1;
    let bits_12 = (instruction >> 31) & 0b1;
    sign_extend(
        (bits_12 << 12) | (bits_11 << 11) | (bits_10_5 << 5) | (bits_4_1 << 1),
        12,
    ) as Immediate
}

fn immediate_u(instruction: u32) -> Immediate {
    (instruction & 0b1111_1111_1111_1111_1111_0000_0000_0000) as Immediate
}

fn immediate_j(instruction: u32) -> Immediate {
    let bits_10_1 = (instruction >> 21) & 0b11_1111_1111;
    let bits_11 = (instruction >> 20) & 0b1;
    let bits_19_12 = (instruction >> 12) & 0b1111_1111;
    let bits_20 = (instruction >> 31) & 0b1;
    sign_extend(
        (bits_20 << 20) | (bits_19_12 << 12) | (bits_11 << 11) | (bits_10_1 << 1),
        20,
    ) as Immediate
}

fn rs1(instruction: u32) -> RS1index {
    ((instruction >> 15) & 0b1_1111) as RS1index
}

fn rs2(instruction: u32) -> RS2index {
    ((instruction >> 20) & 0b1_1111) as RS2index
}

fn rd(instruction: u32) -> RDindex {
    ((instruction >> 7) & 0b1_1111) as RDindex
}

fn funct3(instruction: u32) -> Funct3 {
    ((instruction >> 12) & 0b111) as Funct3
}

fn funct5(instruction: u32) -> Funct3 {
    ((instruction >> 27) & 0b1_1111) as Funct5
}

fn funct7(instruction: u32) -> Funct7 {
    ((instruction >> 25) & 0b111_1111) as Funct7
}

#[derive(Debug)]
pub enum OpCode {
    LOAD,
    LOADFP,
    CUSTOM0,
    MISCMEM,
    OPIMM,
    AUIPC,
    OPIMM32,
    LEN48,
    STORE,
    STOREFP,
    CUSTOM1,
    AMO,
    OP,
    LUI,
    OP32,
    LEN64,
    MADD,
    MSUB,
    NMSUB,
    NMADD,
    OPFP,
    RESERVED1,
    CUSTOM2,
    LEN482,
    BRANCH,
    JALR,
    RESERVED2,
    JAL,
    SYSTEM,
    RESERVED3,
    CUSTOM3,
    LEN80,
}

fn get_opcode(instruction: u32) -> Result<OpCode, &'static str> {
    let op_upper_bits = (instruction >> 5) & 0b11;
    let op_lower_bits = (instruction >> 2) & 0b111;
    match op_upper_bits {
        0b00 => match op_lower_bits {
            0b000 => Ok(OpCode::LOAD),
            0b001 => Ok(OpCode::LOADFP),
            0b010 => Ok(OpCode::CUSTOM0),
            0b011 => Ok(OpCode::MISCMEM),
            0b100 => Ok(OpCode::OPIMM),
            0b101 => Ok(OpCode::AUIPC),
            0b110 => Ok(OpCode::OPIMM32),
            0b111 => Ok(OpCode::LEN48),
            _ => unreachable!(),
        },
        0b01 => match op_lower_bits {
            0b000 => Ok(OpCode::STORE),
            0b001 => Ok(OpCode::STOREFP),
            0b010 => Ok(OpCode::CUSTOM1),
            0b011 => Ok(OpCode::AMO),
            0b100 => Ok(OpCode::OP),
            0b101 => Ok(OpCode::LUI),
            0b110 => Ok(OpCode::OP32),
            0b111 => Ok(OpCode::LEN64),
            _ => unreachable!(),
        },
        0b10 => match op_lower_bits {
            0b000 => Ok(OpCode::MADD),
            0b001 => Ok(OpCode::MSUB),
            0b010 => Ok(OpCode::NMSUB),
            0b011 => Ok(OpCode::NMADD),
            0b100 => Ok(OpCode::OPFP),
            0b101 => Ok(OpCode::RESERVED1),
            0b110 => Ok(OpCode::CUSTOM2),
            0b111 => Ok(OpCode::LEN482),
            _ => unreachable!(),
        },
        0b11 => match op_lower_bits {
            0b000 => Ok(OpCode::BRANCH),
            0b001 => Ok(OpCode::JALR),
            0b010 => Ok(OpCode::RESERVED2),
            0b011 => Ok(OpCode::JAL),
            0b100 => Ok(OpCode::SYSTEM),
            0b101 => Ok(OpCode::RESERVED3),
            0b110 => Ok(OpCode::CUSTOM3),
            0b111 => Ok(OpCode::LEN80),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

#[allow(clippy::too_many_lines)]
pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    let op = get_opcode(instruction)?;

    match op {
        OpCode::LOAD => {
            /* All LOAD are I-Type instructions */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Immediate = immediate_i(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::LB(rd_index, rs1, i_imm)),
                0b001 => Ok(Instruction::LH(rd_index, rs1, i_imm)),
                0b010 => Ok(Instruction::LW(rd_index, rs1, i_imm)),
                0b100 => Ok(Instruction::LBU(rd_index, rs1, i_imm)),
                0b101 => Ok(Instruction::LHU(rd_index, rs1, i_imm)),
                _ => Err("Invalid funct3 I-Type"),
            }
        }
        OpCode::LOADFP => Err("Not implemented: LOADFP"),
        OpCode::CUSTOM0 => Err("Not implemented: CUSTOM0"),
        OpCode::MISCMEM => {
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Immediate = immediate_i(instruction);
            Ok(Instruction::FENCE(rd_index, rs1, i_imm))
        }
        OpCode::OPIMM => {
            /* All OPIMM are I-Type instructions */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Immediate = immediate_i(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::ADDI(rd_index, rs1, i_imm)),
                0b010 => Ok(Instruction::SLTI(rd_index, rs1, i_imm)),
                0b011 => Ok(Instruction::SLTIU(rd_index, rs1, i_imm)),
                0b100 => Ok(Instruction::XORI(rd_index, rs1, i_imm)),
                0b110 => Ok(Instruction::ORI(rd_index, rs1, i_imm)),
                0b111 => Ok(Instruction::ANDI(rd_index, rs1, i_imm)),
                0b001 => Ok(Instruction::SLLI(rd_index, rs1, i_imm)),
                0b101 => {
                    if (i_imm & 0b0100_0000_0000) == 0 {
                        Ok(Instruction::SRLI(rd_index, rs1, i_imm))
                    } else {
                        Ok(Instruction::SRAI(rd_index, rs1, i_imm))
                    }
                }
                _ => unreachable!(),
            }
        }
        OpCode::AUIPC => {
            /* U Type */
            let rd_index: RDindex = rd(instruction);
            let u_imm: Immediate = immediate_u(instruction);
            Ok(Instruction::AUIPC(rd_index, u_imm))
        }
        OpCode::OPIMM32 => Err("Not implemented: OPIMM32"),
        OpCode::LEN48 => Err("Not implemented: LEN48"),
        OpCode::STORE => {
            /* STOREs are S-Type */
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);
            let s_imm: Immediate = immediate_s(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::SB(rs1, rs2, s_imm)),
                0b001 => Ok(Instruction::SH(rs1, rs2, s_imm)),
                0b010 => Ok(Instruction::SW(rs1, rs2, s_imm)),
                _ => Err("Invalid funct3 S-Type"),
            }
        }
        OpCode::STOREFP => Err("Not implemented: STOREFP"),
        OpCode::CUSTOM1 => Err("Not implemented: CUSTOM1"),
        OpCode::AMO => {
            let rd: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);
            match funct5(instruction) {
                0b00010 => Ok(Instruction::LRW(rd, rs1)),
                0b00011 => Ok(Instruction::SCW(rd, rs1, rs2)),
                0b00001 => Ok(Instruction::AMOSWAPW(rd, rs1, rs2)),
                0b00000 => Ok(Instruction::AMOADDW(rd, rs1, rs2)),
                0b00100 => Ok(Instruction::AMOXORW(rd, rs1, rs2)),
                0b01100 => Ok(Instruction::AMOANDW(rd, rs1, rs2)),
                0b01000 => Ok(Instruction::AMOORW(rd, rs1, rs2)),
                0b10000 => Ok(Instruction::AMOMINW(rd, rs1, rs2)),
                0b10100 => Ok(Instruction::AMOMAXW(rd, rs1, rs2)),
                0b11000 => Ok(Instruction::AMOMINUW(rd, rs1, rs2)),
                0b11100 => Ok(Instruction::AMOMAXUW(rd, rs1, rs2)),
                _ => Err("Invalid funct5 AMO-Type"),
            }
        }
        OpCode::OP => {
            /* All OP are R-Type instructions */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);

            let is_m_extension = funct7(instruction) & 0b1 == 1;
            match funct3(instruction) {
                0b000 => {
                    if is_m_extension {
                        return Ok(Instruction::MUL(rd_index, rs1, rs2));
                    }
                    if funct7(instruction) == 0 {
                        Ok(Instruction::ADD(rd_index, rs1, rs2))
                    } else {
                        Ok(Instruction::SUB(rd_index, rs1, rs2))
                    }
                }
                0b001 => {
                    if is_m_extension {
                        return Ok(Instruction::MULH(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::SLL(rd_index, rs1, rs2))
                }
                0b010 => {
                    if is_m_extension {
                        return Ok(Instruction::MULHSU(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::SLT(rd_index, rs1, rs2))
                }
                0b011 => {
                    if is_m_extension {
                        return Ok(Instruction::MULHU(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::SLTU(rd_index, rs1, rs2))
                }
                0b100 => {
                    if is_m_extension {
                        return Ok(Instruction::DIV(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::XOR(rd_index, rs1, rs2))
                }
                0b101 => {
                    if is_m_extension {
                        return Ok(Instruction::DIVU(rd_index, rs1, rs2));
                    }
                    if funct7(instruction) == 0 {
                        Ok(Instruction::SRL(rd_index, rs1, rs2))
                    } else {
                        Ok(Instruction::SRA(rd_index, rs1, rs2))
                    }
                }
                0b110 => {
                    if is_m_extension {
                        return Ok(Instruction::REM(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::OR(rd_index, rs1, rs2))
                }
                0b111 => {
                    if is_m_extension {
                        return Ok(Instruction::REMU(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::AND(rd_index, rs1, rs2))
                }
                _ => unreachable!(),
            }
        }
        OpCode::LUI => {
            /* U Type */
            let rd_index: RDindex = rd(instruction);
            let u_imm: Immediate = immediate_u(instruction);
            Ok(Instruction::LUI(rd_index, u_imm))
        }
        OpCode::OP32 => Err("Not implemented: OP32"),
        OpCode::LEN64 => Err("Not implemented: LEN64"),
        OpCode::MADD => Err("Not implemented: MADD"),
        OpCode::MSUB => Err("Not implemented: MSUB"),
        OpCode::NMSUB => Err("Not implemented: NMSUB"),
        OpCode::NMADD => Err("Not implemented: NMADD"),
        OpCode::OPFP => Err("Not implemented: OPFP"),
        OpCode::RESERVED1 => Err("Not implemented: RESERVED1"),
        OpCode::CUSTOM2 => Err("Not implemented: CUSTOM2"),
        OpCode::LEN482 => Err("Not implemented: LEN482"),
        OpCode::BRANCH => {
            /* B-Type instructions */
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);
            let b_imm: Immediate = immediate_b(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::BEQ(rs1, rs2, b_imm)),
                0b001 => Ok(Instruction::BNE(rs1, rs2, b_imm)),
                0b100 => Ok(Instruction::BLT(rs1, rs2, b_imm)),
                0b101 => Ok(Instruction::BGE(rs1, rs2, b_imm)),
                0b110 => Ok(Instruction::BLTU(rs1, rs2, b_imm)),
                0b111 => Ok(Instruction::BGEU(rs1, rs2, b_imm)),
                _ => Err("Invalid funct3 B-Type"),
            }
        }
        OpCode::JALR => {
            /* I-Type instruction */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Immediate = immediate_i(instruction);
            Ok(Instruction::JALR(rd_index, rs1, i_imm))
        }
        OpCode::RESERVED2 => Err("Not implemented: RESERVED2"),
        OpCode::JAL => {
            let rd_index: RDindex = rd(instruction);
            let j_imm: Immediate = immediate_j(instruction);
            Ok(Instruction::JAL(rd_index, j_imm))
        }
        OpCode::SYSTEM => {
            /* I-Type instruction */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let sys_imm: u32 = immediate_system(instruction);
            match funct3(instruction) {
                0b000 => match sys_imm {
                    0b0000_0000_0000 => Ok(Instruction::ECALL()),
                    0b0000_0000_0001 => Ok(Instruction::EBREAK()),
                    0b0011_0000_0010 => Ok(Instruction::MRET()),
                    0b0001_0000_0101 => Ok(Instruction::WFI()),
                    _ => Err("Invalid SYSTEM instruction immediate"),
                },
                0b001 => Ok(Instruction::CSRRW(rd_index, rs1, sys_imm)),
                0b010 => Ok(Instruction::CSRRS(rd_index, rs1, sys_imm)),
                0b011 => Ok(Instruction::CSRRC(rd_index, rs1, sys_imm)),
                0b101 => {
                    /* This instruction repurposes rs1 as immediate */
                    Ok(Instruction::CSRRWI(rd_index, rs1, sys_imm))
                }
                0b110 => {
                    /* This instruction repurposes rs1 as immediate */
                    Ok(Instruction::CSRRSI(rd_index, rs1, sys_imm))
                }
                0b111 => {
                    /* This instruction repurposes rs1 as immediate */
                    Ok(Instruction::CSRRCI(rd_index, rs1, sys_imm))
                }
                _ => Err("Invalid funct3 I-Type"),
            }
        }
        OpCode::RESERVED3 => Err("Not implemented: RESERVED3"),
        OpCode::CUSTOM3 => Err("Not implemented: CUSTOM3"),
        OpCode::LEN80 => Err("Not implemented: LEN80"),
    }
}
