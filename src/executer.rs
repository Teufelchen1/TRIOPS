use crate::decoder::{Instruction, RS1value, RS2value};
use crate::system::{Memory, RegisterFile};

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

macro_rules! add_signed {
    ($unsigned:expr, $signed:expr) => {{
        if $signed.is_negative() {
            $unsigned.wrapping_sub($signed.unsigned_abs())
        } else {
            $unsigned.wrapping_add($signed.unsigned_abs())
        }
    }};
}

#[allow(clippy::too_many_lines)]
pub fn exec(
    register_file: &mut RegisterFile,
    memory: &mut Memory,
    instruction: &Instruction,
    zicsr_enabled: bool,
    m_enabled: bool,
) -> bool {
    assert!(
        !instruction.is_zicsr() || zicsr_enabled,
        "Zicsr instruction found but zicsr is not enabled."
    );
    assert!(
        !instruction.is_m() || m_enabled,
        "M instruction found but M is not enabled."
    );

    match *instruction {
        Instruction::LUI(rdindex, uimmediate) => {
            register_file.write(rdindex, uimmediate);
        }
        Instruction::AUIPC(rdindex, uimmediate) => {
            register_file.write(rdindex, register_file.pc.wrapping_add(uimmediate));
        }
        Instruction::JAL(rdindex, jimmediate) => {
            let sign_imm = sign_extend(jimmediate, 20) as i32;
            register_file.write(rdindex, register_file.pc + 4);
            assert!(
                (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                "JAL target addr not 4 byte aligned."
            );
            register_file.pc = add_signed!(register_file.pc, sign_imm);
            return true;
        }
        Instruction::JALR(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) & !0b1;
            assert!(target % 4 == 0, "JALR target addr not 4 byte aligned.");
            register_file.write(rdindex, register_file.pc + 4);
            register_file.pc = target;
            return true;
        }
        Instruction::BEQ(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if rs1 == rs2 {
                assert!(
                    (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return true;
            }
        }
        Instruction::BNE(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if rs1 != rs2 {
                assert!(
                    (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return true;
            }
        }
        Instruction::BLT(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if (rs1 as i32) < (rs2 as i32) {
                assert!(
                    (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return true;
            }
        }
        Instruction::BGE(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if (rs1 as i32) >= (rs2 as i32) {
                assert!(
                    (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return true;
            }
        }
        Instruction::BLTU(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if rs1 < rs2 {
                assert!(
                    (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return true;
            }
        }
        Instruction::BGEU(rs1index, rs2index, bimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if rs1 >= rs2 {
                assert!(
                    (add_signed!(register_file.pc, sign_imm) % 4) == 0,
                    "Branch target addr not 4 byte aligned."
                );
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return true;
            }
        }
        Instruction::LB(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            let value = sign_extend(memory.read_byte(target), 8);
            register_file.write(rdindex, value);
        }
        Instruction::LH(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            let value = sign_extend(memory.read_halfword(target), 16);
            register_file.write(rdindex, value);
        }
        Instruction::LW(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            let value = memory.read_word(target);
            register_file.write(rdindex, value);
        }
        Instruction::LBU(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            let value = memory.read_byte(target);
            register_file.write(rdindex, value);
        }
        Instruction::LHU(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            let value = memory.read_halfword(target);
            register_file.write(rdindex, value);
        }
        Instruction::SB(rs1index, rs2index, simmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(simmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            memory.write_byte(target, rs2);
        }
        Instruction::SH(rs1index, rs2index, simmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(simmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            memory.write_halfword(target, rs2);
        }
        Instruction::SW(rs1index, rs2index, simmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(simmediate, 12) as i32;
            let target = add_signed!(rs1, sign_imm) as usize;
            //println!("{:}, {:}, {:}", rs1, rs2, sign_imm);
            memory.write_word(target, rs2);
        }
        Instruction::ADDI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            //println!("{:b}, {:b}, {:}", iimmediate, sign_imm, sign_imm);
            register_file.write(rdindex, add_signed!(rs1, sign_imm));
        }
        Instruction::SLTI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12);
            if (rs1 as i32) < (sign_imm as i32) {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::SLTIU(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12);
            if rs1 < sign_imm {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::XORI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 ^ sign_extend(iimmediate, 12));
        }
        Instruction::ORI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 | sign_extend(iimmediate, 12));
        }
        Instruction::ANDI(rdindex, rs1index, iimmediate) => {
            let rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, rs1 & sign_extend(iimmediate, 12));
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
            let shamt = iimmediate & 0b1_1111;
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
            register_file.csr.mcause = 11; /* Environment call from M-Mode */
            register_file.pc = register_file.csr.mtvec;
        }
        Instruction::EBREAK() => {
            return false;
        }
        Instruction::MRET() => {
            register_file.pc = register_file.csr.mepc;
        }
        Instruction::CSRRW(rd_index, rs1, i_imm) => {
            if rd_index != 0 {
                register_file.write(rd_index, register_file.csr.read(i_imm));
                register_file.csr.write(i_imm, register_file.read(rs1));
            }
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
            /* rs1 is actual an immediate */
            let uimm = u32::try_from(rs1).unwrap();
            if rd_index != 0 {
                register_file.write(rd_index, register_file.csr.read(i_imm));
            }
            register_file.csr.write(i_imm, uimm);
        }
        Instruction::CSRRSI(rd_index, rs1, i_imm) => {
            /* rs1 is actual an immediate */
            let uimm = u32::try_from(rs1).unwrap();
            let csr_value = register_file.csr.read(i_imm);
            register_file.write(rd_index, csr_value);
            if uimm != 0 {
                register_file.csr.write(i_imm, uimm | csr_value);
            }
        }
        Instruction::CSRRCI(rd_index, rs1, i_imm) => {
            /* rs1 is actual an immediate */
            let uimm = u32::try_from(rs1).unwrap();
            let csr_value = register_file.csr.read(i_imm);
            register_file.write(rd_index, csr_value);
            if uimm != 0 {
                register_file.csr.write(i_imm, !uimm & csr_value);
            }
        }
        Instruction::MUL(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            // Rust panics if the result of the multiplication overflows. The RISC-V spec doesn't care and just stores the low 32 bits
            // For this reason, the multiplication is done on 64-bit numbers and then typecasted.
            let _rs1_64 = _rs1 as u64;
            let _rs2_64 = _rs2 as u64;
            register_file.write(rdindex, (_rs1_64* _rs2_64) as u32);
        }
        Instruction::MULH(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let result: i64 = (i64::from(_rs1)) * (i64::from(_rs2));
            let high_bytes: u32 = (result >> 32) as u32;
            register_file.write(rdindex, high_bytes);
        }
        Instruction::MULHSU(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let result = i64::from(_rs1).wrapping_mul(i64::from(_rs2));
            let high_bytes: u32 = (result >> 32) as u32;
            register_file.write(rdindex, high_bytes);
        }
        Instruction::MULHU(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let result: u64 = (_rs1 as u64 * _rs2 as u64);
            let high_bytes: u32 = (result >> 32) as u32;
            register_file.write(rdindex, high_bytes);
        }
        Instruction::DIV(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            if (_rs2 == 0) {
                // The spec defines that -1 should be stored. In 32-bit two's complement, u32::MAX is -1
                register_file.write(rdindex, u32::MAX);
            }
            else {
                let result: i32 = (_rs1 / _rs2) as i32;
                register_file.write(rdindex, result as u32);
            } 
        }
        Instruction::DIVU(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            if (_rs2 == 0) {
                register_file.write(rdindex, u32::MAX);
            }
            else {
                register_file.write(rdindex, _rs1 / _rs2);
            }
        }
        Instruction::REM(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            if (_rs2 == 0) {
                register_file.write(rdindex, _rs1);
            }
            else {
                let result: i32 = (_rs1 % _rs2) as i32;
                register_file.write(rdindex, result as u32);
            }
        }
        Instruction::REMU(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            if (_rs2 == 0) {
                register_file.write(rdindex, _rs1);
            }
            else {
                register_file.write(rdindex, _rs1 % _rs2);
            }
        }
    }
    register_file.pc += 4;
    true
}
