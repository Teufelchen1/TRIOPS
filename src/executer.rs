//! This file is scoped to a single function: `exec()`.
use std::cmp::{max, min};

use crate::instructions::{sign_extend, Instruction, RS1value, RS2value};
use crate::memory::Memory;
use crate::register::Register;

macro_rules! add_signed {
    ($unsigned:expr, $signed:expr) => {{
        if $signed.is_negative() {
            $unsigned.wrapping_sub($signed.unsigned_abs())
        } else {
            $unsigned.wrapping_add($signed.unsigned_abs())
        }
    }};
}

/// Executes one instruction.
#[allow(clippy::too_many_lines)]
pub fn exec(
    register_file: &mut Register,
    memory: &mut Memory,
    instruction: &Instruction,
    zicsr_enabled: bool,
    m_enabled: bool,
) -> anyhow::Result<()> {
    assert!(
        !instruction.is_zicsr() || zicsr_enabled,
        "Zicsr instruction found but zicsr is not enabled."
    );
    assert!(
        !instruction.is_m() || m_enabled,
        "M instruction found but M is not enabled."
    );

    // Compressed instructions must be decompressed first, by
    // doing so, they are expanded to a regular instruction.
    // Because the PC is advanced based on instruction size (bytes)
    // we also have to differenciate that here.
    let compressed_instruction;
    let instruction_address = register_file.pc;
    assert!(
        instruction_address % 2 == 0,
        "Instruction address not aligned on two byte."
    );
    let actual_instruction = {
        if instruction.is_compressed() {
            register_file.pc += 2;
            compressed_instruction = instruction.decompress();
            &compressed_instruction
        } else {
            register_file.pc += 4;
            instruction
        }
    };

    match *actual_instruction {
        Instruction::LUI(rdindex, uimmediate) => {
            register_file.write(rdindex, uimmediate as u32);
        }
        Instruction::AUIPC(rdindex, uimmediate) => {
            register_file.write(
                rdindex,
                (register_file.pc - 4).wrapping_add(uimmediate as u32),
            );
        }
        Instruction::JAL(rdindex, jimmediate) => {
            register_file.write(rdindex, register_file.pc);
            assert!(
                (add_signed!(register_file.pc, jimmediate) % 2) == 0,
                "JAL target addr not 2 byte aligned."
            );
            register_file.pc = add_signed!(instruction_address, jimmediate);
        }
        Instruction::JALR(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let target = add_signed!(rs1, iimmediate) & !0b1;
            assert!(target % 2 == 0, "JALR target addr not 4 byte aligned.");
            register_file.write(rdindex, register_file.pc);
            register_file.pc = target;
        }
        Instruction::BEQ(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs1 == rs2 {
                assert!(
                    (add_signed!(instruction_address, bimmediate) % 2) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(instruction_address, bimmediate);
            }
        }
        Instruction::BNE(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs1 != rs2 {
                assert!(
                    (add_signed!(instruction_address, bimmediate) % 2) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(instruction_address, bimmediate);
            }
        }
        Instruction::BLT(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if (rs1 as i32) < (rs2 as i32) {
                assert!(
                    (add_signed!(instruction_address, bimmediate) % 2) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(instruction_address, bimmediate);
            }
        }
        Instruction::BGE(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if (rs1 as i32) >= (rs2 as i32) {
                assert!(
                    (add_signed!(instruction_address, bimmediate) % 2) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(instruction_address, bimmediate);
            }
        }
        Instruction::BLTU(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs1 < rs2 {
                assert!(
                    (add_signed!(instruction_address, bimmediate) % 2) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(instruction_address, bimmediate);
            }
        }
        Instruction::BGEU(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs1 >= rs2 {
                assert!(
                    (add_signed!(instruction_address, bimmediate) % 2) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(instruction_address, bimmediate);
            }
        }
        Instruction::LB(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let target = add_signed!(rs1, iimmediate) as usize;
            let value = sign_extend(memory.read_byte(target)?, 8);
            register_file.write(rdindex, value);
        }
        Instruction::LH(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let target = add_signed!(rs1, iimmediate) as usize;
            let value = sign_extend(memory.read_halfword(target)?, 16);
            register_file.write(rdindex, value);
        }
        Instruction::LW(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let target = add_signed!(rs1, iimmediate) as usize;
            let value = memory.read_word(target)?;
            register_file.write(rdindex, value);
        }
        Instruction::LBU(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let target = add_signed!(rs1, iimmediate) as usize;
            let value = memory.read_byte(target)?;
            register_file.write(rdindex, value);
        }
        Instruction::LHU(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let target = add_signed!(rs1, iimmediate) as usize;
            let value = memory.read_halfword(target)?;
            register_file.write(rdindex, value);
        }
        Instruction::SB(rs1index, rs2index, simmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let target = add_signed!(rs1, simmediate) as usize;
            memory.write_byte(target, rs2);
        }
        Instruction::SH(rs1index, rs2index, simmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let target = add_signed!(rs1, simmediate) as usize;
            memory.write_halfword(target, rs2);
        }
        Instruction::SW(rs1index, rs2index, simmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let target = add_signed!(rs1, simmediate) as usize;
            memory.write_word(target, rs2);
        }
        Instruction::ADDI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, add_signed!(rs1, iimmediate));
        }
        Instruction::SLTI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            if (rs1 as i32) < iimmediate {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::SLTIU(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            if rs1 < iimmediate as u32 {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::XORI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 ^ iimmediate as u32);
        }
        Instruction::ORI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 | iimmediate as u32);
        }
        Instruction::ANDI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 & iimmediate as u32);
        }
        Instruction::SLLI(rdindex, rs1index, iimmediate) => {
            let shamt = iimmediate & 0b1_1111;
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 << shamt);
        }
        Instruction::SRLI(rdindex, rs1index, iimmediate) => {
            let shamt = iimmediate & 0b1_1111;
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 >> shamt);
        }
        Instruction::SRAI(rdindex, rs1index, iimmediate) => {
            let shamt = (iimmediate & 0b1_1111) as u32;
            let rs1: RS1value = register_file.read(rs1index);
            let value = sign_extend(rs1 >> shamt, 32 - shamt);
            register_file.write(rdindex, value);
        }
        Instruction::ADD(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1.wrapping_add(rs2));
        }
        Instruction::SUB(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1.wrapping_sub(rs2));
        }
        Instruction::SLL(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1 << (rs2 & 0b1_1111));
        }
        Instruction::SLT(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if (rs1 as i32) < (rs2 as i32) {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::SLTU(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs1 < rs2 {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::XOR(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1 ^ rs2);
        }
        Instruction::SRL(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1 >> (rs2 & 0b1_1111));
        }
        Instruction::SRA(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let shamt = rs2 & 0b1_1111;
            register_file.write(rdindex, sign_extend(rs1 >> shamt, 32 - shamt));
        }
        Instruction::OR(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1 | rs2);
        }
        Instruction::AND(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, rs1 & rs2);
        }
        Instruction::FENCE(_rdindex, _rs1index, _iimmediate) => { /* Nop */ }
        Instruction::ECALL() => {
            register_file.csr.mepc = register_file.pc;
            register_file.csr.mcause = 11; // Environment call from M-Mode
            register_file.pc = register_file.csr.mtvec;
        }
        Instruction::EBREAK() => { /* Nop */ }
        Instruction::MRET() => {
            register_file.pc = register_file.csr.mepc;
        }
        Instruction::CSRRW(rd_index, rs1, i_imm) => {
            register_file.write(rd_index, register_file.csr.read(i_imm));
            register_file.csr.write(i_imm, register_file.read(rs1));
        }
        Instruction::CSRRS(rd_index, rs1, i_imm) => {
            let csr_value = register_file.csr.read(i_imm);
            register_file.write(rd_index, csr_value);
            if rs1 != 0 {
                register_file
                    .csr
                    .write(i_imm, register_file.read(rs1) | csr_value);
            }
        }
        Instruction::CSRRC(rd_index, rs1, i_imm) => {
            let csr_value = register_file.csr.read(i_imm);
            register_file.write(rd_index, csr_value);
            if rs1 != 0 {
                register_file
                    .csr
                    .write(i_imm, !register_file.read(rs1) & csr_value);
            }
        }
        Instruction::CSRRWI(rd_index, rs1, i_imm) => {
            // rs1 is actual an immediate
            let uimm = u32::try_from(rs1).unwrap();
            if rd_index != 0 {
                register_file.write(rd_index, register_file.csr.read(i_imm));
            }
            register_file.csr.write(i_imm, uimm);
        }
        Instruction::CSRRSI(rd_index, rs1, i_imm) => {
            // rs1 is actual an immediate
            let uimm = u32::try_from(rs1).unwrap();
            let csr_value = register_file.csr.read(i_imm);
            register_file.write(rd_index, csr_value);
            if uimm != 0 {
                register_file.csr.write(i_imm, uimm | csr_value);
            }
        }
        Instruction::CSRRCI(rd_index, rs1, i_imm) => {
            // rs1 is actual an immediate
            let uimm = u32::try_from(rs1).unwrap();
            let csr_value = register_file.csr.read(i_imm);
            register_file.write(rd_index, csr_value);
            if uimm != 0 {
                register_file.csr.write(i_imm, !uimm & csr_value);
            }
        }
        Instruction::MUL(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            // Rust panics if the result of the multiplication overflows.
            // The RISC-V spec doesn't care and just stores the low 32 bits
            // For this reason, the multiplication is done on 64-bit numbers and then typecasted.
            let rs1_64 = u64::from(rs1);
            let rs2_64 = u64::from(rs2);
            register_file.write(rdindex, (rs1_64 * rs2_64) as u32);
        }
        Instruction::MULH(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let result: i64 = (i64::from(rs1 as i32)) * (i64::from(rs2 as i32));
            let high_bytes: u32 = (result >> 32) as u32;
            register_file.write(rdindex, high_bytes);
        }
        Instruction::MULHSU(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let result = i64::from(rs1 as i32).wrapping_mul(i64::from(rs2));
            let high_bytes: u32 = (result >> 32) as u32;
            register_file.write(rdindex, high_bytes);
        }
        Instruction::MULHU(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let result: u64 = u64::from(rs1) * u64::from(rs2);
            let high_bytes: u32 = (result >> 32) as u32;
            register_file.write(rdindex, high_bytes);
        }
        Instruction::DIV(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs2 == 0 {
                // The spec defines that -1 should be stored.
                // In 32-bit two's complement, u32::MAX is -1
                register_file.write(rdindex, u32::MAX);
            } else {
                let result = (rs1 as i32).overflowing_div(rs2 as i32);
                register_file.write(rdindex, result.0 as u32);
            }
        }
        Instruction::DIVU(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs2 == 0 {
                register_file.write(rdindex, u32::MAX);
            } else {
                register_file.write(rdindex, rs1 / rs2);
            }
        }
        Instruction::REM(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs2 == 0 {
                register_file.write(rdindex, rs1);
            } else {
                let result = (rs1 as i32).overflowing_rem(rs2 as i32);
                register_file.write(rdindex, result.0 as u32);
            }
        }
        Instruction::REMU(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            if rs2 == 0 {
                register_file.write(rdindex, rs1);
            } else {
                register_file.write(rdindex, rs1 % rs2);
            }
        }
        Instruction::LRW(rdindex, rs1index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let addr = rs1 as usize;
            let value = memory.read_word(addr)?;
            register_file.write(rdindex, value);
            memory.reservation = Some((addr, value));
        }
        Instruction::SCW(rdindex, rs1index, rs2index) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS1value = register_file.read(rs2index);
            let addr = rs1 as usize;

            let value = memory.read_word(addr)?;

            register_file.write(rdindex, 1);
            if let Some(reservation) = memory.reservation {
                if reservation.0 == addr && reservation.1 == value {
                    memory.write_word(addr, rs2);
                    register_file.write(rdindex, 0);
                }
            }
            memory.reservation = None;
        }
        Instruction::AMOSWAPW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = org;
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOADDW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = (data as i32).wrapping_add(org as i32);
            memory.write_word(addr_rs1 as usize, result as u32);
        }
        Instruction::AMOXORW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = data ^ org;
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOANDW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = data & org;
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOORW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = data | org;
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOMINW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = min(data as i32, org as i32) as u32;
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOMAXW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = max(data as i32, org as i32) as u32;
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOMINUW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = min(data, org);
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::AMOMAXUW(rdindex, rs1index, rs2index) => {
            let addr_rs1: RS1value = register_file.read(rs1index);
            let org: RS2value = register_file.read(rs2index);
            let data = memory.read_word(addr_rs1 as usize)?;
            register_file.write(rdindex, data);
            let result = max(data, org);
            memory.write_word(addr_rs1 as usize, result);
        }
        Instruction::WFI() => {
            // Temporal hack to make the RIOT experience nicer
            register_file.csr.mie = 1;
            register_file.pc -= 4;
        }
        _ => todo!("{:?}", actual_instruction),
    }
    Ok(())
}
