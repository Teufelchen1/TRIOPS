//! This file is scoped to a single function: `exec()`.
use std::cmp::{max, min};

use super::{AddrBus, CPU};
use crate::instructions::{sign_extend, Instruction, RS1value, RS2value};

macro_rules! add_signed {
    ($unsigned:expr, $signed:expr) => {{
        if $signed.is_negative() {
            $unsigned.wrapping_sub($signed.unsigned_abs())
        } else {
            $unsigned.wrapping_add($signed.unsigned_abs())
        }
    }};
}

impl<T: AddrBus> CPU<T> {
    /// Executes one instruction.
    #[allow(clippy::too_many_lines)]
    pub fn exec(
        &mut self,
        // self.register: &mut Register,
        // self.memory: &mut Memory,
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
        let instruction_address = self.register.pc;
        assert!(
            instruction_address.is_multiple_of(2),
            "Instruction address not aligned on two byte."
        );
        let actual_instruction = {
            if instruction.is_compressed() {
                self.register.pc += 2;
                compressed_instruction = instruction.decompress();
                &compressed_instruction
            } else {
                self.register.pc += 4;
                instruction
            }
        };

        match *actual_instruction {
            Instruction::LUI(rdindex, uimmediate) => {
                self.register.write(rdindex, uimmediate as u32);
            }
            Instruction::AUIPC(rdindex, uimmediate) => {
                self.register.write(
                    rdindex,
                    (self.register.pc - 4).wrapping_add(uimmediate as u32),
                );
            }
            Instruction::JAL(rdindex, jimmediate) => {
                self.register.write(rdindex, self.register.pc);
                assert!(
                    (add_signed!(self.register.pc, jimmediate) % 2) == 0,
                    "JAL target addr not 2 byte aligned."
                );
                self.register.pc = add_signed!(instruction_address, jimmediate);
            }
            Instruction::JALR(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let target = add_signed!(rs1, iimmediate) & !0b1;
                assert!(target % 2 == 0, "JALR target addr not 4 byte aligned.");
                self.register.write(rdindex, self.register.pc);
                self.register.pc = target;
            }
            Instruction::BEQ(rs1index, rs2index, bimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs1 == rs2 {
                    assert!(
                        (add_signed!(instruction_address, bimmediate) % 2) == 0,
                        "Branch target addr not 4 byte aligned."
                    );
                    self.register.pc = add_signed!(instruction_address, bimmediate);
                }
            }
            Instruction::BNE(rs1index, rs2index, bimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs1 != rs2 {
                    assert!(
                        (add_signed!(instruction_address, bimmediate) % 2) == 0,
                        "Branch target addr not 4 byte aligned."
                    );
                    self.register.pc = add_signed!(instruction_address, bimmediate);
                }
            }
            Instruction::BLT(rs1index, rs2index, bimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if (rs1 as i32) < (rs2 as i32) {
                    assert!(
                        (add_signed!(instruction_address, bimmediate) % 2) == 0,
                        "Branch target addr not 4 byte aligned."
                    );
                    self.register.pc = add_signed!(instruction_address, bimmediate);
                }
            }
            Instruction::BGE(rs1index, rs2index, bimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if (rs1 as i32) >= (rs2 as i32) {
                    assert!(
                        (add_signed!(instruction_address, bimmediate) % 2) == 0,
                        "Branch target addr not 4 byte aligned."
                    );
                    self.register.pc = add_signed!(instruction_address, bimmediate);
                }
            }
            Instruction::BLTU(rs1index, rs2index, bimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs1 < rs2 {
                    assert!(
                        (add_signed!(instruction_address, bimmediate) % 2) == 0,
                        "Branch target addr not 4 byte aligned."
                    );
                    self.register.pc = add_signed!(instruction_address, bimmediate);
                }
            }
            Instruction::BGEU(rs1index, rs2index, bimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs1 >= rs2 {
                    assert!(
                        (add_signed!(instruction_address, bimmediate) % 2) == 0,
                        "Branch target addr not 4 byte aligned."
                    );
                    self.register.pc = add_signed!(instruction_address, bimmediate);
                }
            }
            Instruction::LB(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let target = add_signed!(rs1, iimmediate) as usize;
                let value = sign_extend(self.memory.read_byte(target)?, 8);
                self.register.write(rdindex, value);
            }
            Instruction::LH(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let target = add_signed!(rs1, iimmediate) as usize;
                let value = sign_extend(self.memory.read_halfword(target)?, 16);
                self.register.write(rdindex, value);
            }
            Instruction::LW(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let target = add_signed!(rs1, iimmediate) as usize;
                let value = self.memory.read_word(target)?;
                self.register.write(rdindex, value);
            }
            Instruction::LBU(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let target = add_signed!(rs1, iimmediate) as usize;
                let value = self.memory.read_byte(target)?;
                self.register.write(rdindex, value);
            }
            Instruction::LHU(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let target = add_signed!(rs1, iimmediate) as usize;
                let value = self.memory.read_halfword(target)?;
                self.register.write(rdindex, value);
            }
            Instruction::SB(rs1index, rs2index, simmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let target = add_signed!(rs1, simmediate) as usize;
                self.memory.write_byte(target, rs2)?;
            }
            Instruction::SH(rs1index, rs2index, simmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let target = add_signed!(rs1, simmediate) as usize;
                self.memory.write_halfword(target, rs2)?;
            }
            Instruction::SW(rs1index, rs2index, simmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let target = add_signed!(rs1, simmediate) as usize;
                self.memory.write_word(target, rs2)?;
            }
            Instruction::ADDI(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                self.register.write(rdindex, add_signed!(rs1, iimmediate));
            }
            Instruction::SLTI(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                if (rs1 as i32) < iimmediate {
                    self.register.write(rdindex, 1);
                } else {
                    self.register.write(rdindex, 0);
                }
            }
            Instruction::SLTIU(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                if rs1 < iimmediate as u32 {
                    self.register.write(rdindex, 1);
                } else {
                    self.register.write(rdindex, 0);
                }
            }
            Instruction::XORI(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                self.register.write(rdindex, rs1 ^ iimmediate as u32);
            }
            Instruction::ORI(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                self.register.write(rdindex, rs1 | iimmediate as u32);
            }
            Instruction::ANDI(rdindex, rs1index, iimmediate) => {
                let rs1: RS1value = self.register.read(rs1index);
                self.register.write(rdindex, rs1 & iimmediate as u32);
            }
            Instruction::SLLI(rdindex, rs1index, iimmediate) => {
                let shamt = iimmediate & 0b1_1111;
                let rs1: RS1value = self.register.read(rs1index);
                self.register.write(rdindex, rs1 << shamt);
            }
            Instruction::SRLI(rdindex, rs1index, iimmediate) => {
                let shamt = iimmediate & 0b1_1111;
                let rs1: RS1value = self.register.read(rs1index);
                self.register.write(rdindex, rs1 >> shamt);
            }
            Instruction::SRAI(rdindex, rs1index, iimmediate) => {
                let shamt = (iimmediate & 0b1_1111) as u32;
                let rs1: RS1value = self.register.read(rs1index);
                let value = sign_extend(rs1 >> shamt, 32 - shamt);
                self.register.write(rdindex, value);
            }
            Instruction::ADD(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1.wrapping_add(rs2));
            }
            Instruction::SUB(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1.wrapping_sub(rs2));
            }
            Instruction::SLL(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1 << (rs2 & 0b1_1111));
            }
            Instruction::SLT(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if (rs1 as i32) < (rs2 as i32) {
                    self.register.write(rdindex, 1);
                } else {
                    self.register.write(rdindex, 0);
                }
            }
            Instruction::SLTU(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs1 < rs2 {
                    self.register.write(rdindex, 1);
                } else {
                    self.register.write(rdindex, 0);
                }
            }
            Instruction::XOR(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1 ^ rs2);
            }
            Instruction::SRL(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1 >> (rs2 & 0b1_1111));
            }
            Instruction::SRA(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let shamt = rs2 & 0b1_1111;
                self.register
                    .write(rdindex, sign_extend(rs1 >> shamt, 32 - shamt));
            }
            Instruction::OR(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1 | rs2);
            }
            Instruction::AND(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                self.register.write(rdindex, rs1 & rs2);
            }
            Instruction::FENCE(_rdindex, _rs1index, _iimmediate) => { /* Nop */ }
            Instruction::ECALL() => {
                self.register.csr.mepc = instruction_address;
                self.register.csr.mcause = 11; // Environment call from M-Mode
                self.register.pc = self.register.csr.mtvec;
            }
            Instruction::EBREAK() => { /* Nop */ }
            Instruction::MRET() => {
                self.register.pc = self.register.csr.mepc;
            }
            Instruction::CSRRW(rd_index, rs1, i_imm) => {
                self.register.write(rd_index, self.register.csr.read(i_imm));
                self.register.csr.write(i_imm, self.register.read(rs1));
            }
            Instruction::CSRRS(rd_index, rs1, i_imm) => {
                let csr_value = self.register.csr.read(i_imm);
                self.register.write(rd_index, csr_value);
                if rs1 != 0 {
                    self.register
                        .csr
                        .write(i_imm, self.register.read(rs1) | csr_value);
                }
            }
            Instruction::CSRRC(rd_index, rs1, i_imm) => {
                let csr_value = self.register.csr.read(i_imm);
                self.register.write(rd_index, csr_value);
                if rs1 != 0 {
                    self.register
                        .csr
                        .write(i_imm, !self.register.read(rs1) & csr_value);
                }
            }
            Instruction::CSRRWI(rd_index, rs1, i_imm) => {
                // rs1 is actual an immediate
                let uimm = u32::try_from(rs1).unwrap();
                if rd_index != 0 {
                    self.register.write(rd_index, self.register.csr.read(i_imm));
                }
                self.register.csr.write(i_imm, uimm);
            }
            Instruction::CSRRSI(rd_index, rs1, i_imm) => {
                // rs1 is actual an immediate
                let uimm = u32::try_from(rs1).unwrap();
                let csr_value = self.register.csr.read(i_imm);
                self.register.write(rd_index, csr_value);
                if uimm != 0 {
                    self.register.csr.write(i_imm, uimm | csr_value);
                }
            }
            Instruction::CSRRCI(rd_index, rs1, i_imm) => {
                // rs1 is actual an immediate
                let uimm = u32::try_from(rs1).unwrap();
                let csr_value = self.register.csr.read(i_imm);
                self.register.write(rd_index, csr_value);
                if uimm != 0 {
                    self.register.csr.write(i_imm, !uimm & csr_value);
                }
            }
            Instruction::MUL(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                // Rust panics if the result of the multiplication overflows.
                // The RISC-V spec doesn't care and just stores the low 32 bits
                // For this reason, the multiplication is done on 64-bit numbers and then typecasted.
                let rs1_64 = u64::from(rs1);
                let rs2_64 = u64::from(rs2);
                self.register.write(rdindex, (rs1_64 * rs2_64) as u32);
            }
            Instruction::MULH(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let result: i64 = (i64::from(rs1 as i32)) * (i64::from(rs2 as i32));
                let high_bytes: u32 = (result >> 32) as u32;
                self.register.write(rdindex, high_bytes);
            }
            Instruction::MULHSU(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let result = i64::from(rs1 as i32).wrapping_mul(i64::from(rs2));
                let high_bytes: u32 = (result >> 32) as u32;
                self.register.write(rdindex, high_bytes);
            }
            Instruction::MULHU(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                let result: u64 = u64::from(rs1) * u64::from(rs2);
                let high_bytes: u32 = (result >> 32) as u32;
                self.register.write(rdindex, high_bytes);
            }
            Instruction::DIV(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs2 == 0 {
                    // The spec defines that -1 should be stored.
                    // In 32-bit two's complement, u32::MAX is -1
                    self.register.write(rdindex, u32::MAX);
                } else {
                    let result = (rs1 as i32).overflowing_div(rs2 as i32);
                    self.register.write(rdindex, result.0 as u32);
                }
            }
            Instruction::DIVU(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs2 == 0 {
                    self.register.write(rdindex, u32::MAX);
                } else {
                    self.register.write(rdindex, rs1 / rs2);
                }
            }
            Instruction::REM(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs2 == 0 {
                    self.register.write(rdindex, rs1);
                } else {
                    let result = (rs1 as i32).overflowing_rem(rs2 as i32);
                    self.register.write(rdindex, result.0 as u32);
                }
            }
            Instruction::REMU(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS2value = self.register.read(rs2index);
                if rs2 == 0 {
                    self.register.write(rdindex, rs1);
                } else {
                    self.register.write(rdindex, rs1 % rs2);
                }
            }
            Instruction::LRW(rdindex, rs1index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let addr = rs1 as usize;
                let value = self.memory.read_word(addr)?;
                self.register.write(rdindex, value);
                //self.memory.reservation = Some((addr, value));
                self.memory.set_reservation(addr, value);
            }
            Instruction::SCW(rdindex, rs1index, rs2index) => {
                let rs1: RS1value = self.register.read(rs1index);
                let rs2: RS1value = self.register.read(rs2index);
                let addr = rs1 as usize;

                let value = self.memory.read_word(addr)?;

                self.register.write(rdindex, 1);
                //if let Some(reservation) = self.memory.reservation {
                if let Some(reservation) = self.memory.get_reservation() {
                    if reservation.0 == addr && reservation.1 == value {
                        self.memory.write_word(addr, rs2)?;
                        self.register.write(rdindex, 0);
                    }
                }
                //self.memory.reservation = None;
                self.memory.del_reservation();
            }
            Instruction::AMOSWAPW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = org;
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOADDW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = (data as i32).wrapping_add(org as i32);
                self.memory.write_word(addr_rs1 as usize, result as u32)?;
            }
            Instruction::AMOXORW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = data ^ org;
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOANDW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = data & org;
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOORW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = data | org;
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOMINW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = min(data as i32, org as i32) as u32;
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOMAXW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = max(data as i32, org as i32) as u32;
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOMINUW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = min(data, org);
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::AMOMAXUW(rdindex, rs1index, rs2index) => {
                let addr_rs1: RS1value = self.register.read(rs1index);
                let org: RS2value = self.register.read(rs2index);
                let data = self.memory.read_word(addr_rs1 as usize)?;
                self.register.write(rdindex, data);
                let result = max(data, org);
                self.memory.write_word(addr_rs1 as usize, result)?;
            }
            Instruction::WFI() => {
                self.waits_for_interrupt = true;
            }
            _ => todo!("{:?}", actual_instruction),
        }
        Ok(())
    }
}
