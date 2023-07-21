use crate::decoder::{Instruction, RS1value, RS2value};
use crate::system::{Memory, RegisterFile};

fn sign_extend(num: u32, bitnum: u32) -> u32 {
    let msb = num >> (bitnum - 1);
    let sign_filled = {
        if msb != 0 {
            !0x0 << bitnum
        } else {
            0x0
        }
    };
    return sign_filled | num;
}

macro_rules! add_signed {
    ($unsigned:expr, $signed:expr) => {{
        if $signed.is_negative() {
            $unsigned - $signed.unsigned_abs()
        } else {
            $unsigned + $signed.unsigned_abs()
        }
    }};
}

pub fn exec(register_file: &mut RegisterFile, memory: &mut Memory, instruction: Instruction, zicsr_enabled: bool, m_enabled: bool) {
	if instruction.is_zicsr() && !zicsr_enabled {
		panic!();
	}
	if instruction.is_m() && !m_enabled {
		panic!();
	}

    match instruction {
        Instruction::LUI(rdindex, uimmediate) => {
            register_file.write(rdindex, uimmediate);
        }
        Instruction::AUIPC(rdindex, uimmediate) => {
            register_file.write(rdindex, register_file.pc + uimmediate);
        }
        Instruction::JAL(rdindex, jimmediate) => {
            let sign_imm = sign_extend(jimmediate, 20) as i32;
            register_file.write(rdindex, register_file.pc + 4);
            if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                panic!("JAL target addr not 4 byte aligned.");
            }
            register_file.pc = add_signed!(register_file.pc, sign_imm);
            return;
        }
        Instruction::JALR(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) & !0b1;
            if target % 4 != 0 {
                panic!("JALR target addr not 4 byte aligned.");
            }
            register_file.write(rdindex, register_file.pc + 4);
            register_file.pc = target;
            return;
        }
        Instruction::BEQ(rs1index, rs2index, bimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if _rs1 == _rs2 {
                if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                    panic!("Branch target addr not 4 byte aligned.");
                }
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return;
            }
        }
        Instruction::BNE(rs1index, rs2index, bimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if _rs1 != _rs2 {
                if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                    panic!("Branch target addr not 4 byte aligned.");
                }
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return;
            }
        }
        Instruction::BLT(rs1index, rs2index, bimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if (_rs1 as i32) < (_rs2 as i32) {
                if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                    panic!("Branch target addr not 4 byte aligned.");
                }
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return;
            }
        }
        Instruction::BGE(rs1index, rs2index, bimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if (_rs1 as i32) >= (_rs2 as i32) {
                if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                    panic!("Branch target addr not 4 byte aligned.");
                }
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return;
            }
        }
        Instruction::BLTU(rs1index, rs2index, bimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if _rs1 < _rs2 {
                if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                    panic!("Branch target addr not 4 byte aligned.");
                }
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return;
            }
        }
        Instruction::BGEU(rs1index, rs2index, bimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(bimmediate, 12) as i32;
            if _rs1 >= _rs2 {
                if (add_signed!(register_file.pc, sign_imm) % 4) != 0 {
                    panic!("Branch target addr not 4 byte aligned.");
                }
                register_file.pc = add_signed!(register_file.pc, sign_imm);
                return;
            }
        }
        Instruction::LB(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            let value = sign_extend(memory.read_byte(target), 8);
            register_file.write(rdindex, value);
        }
        Instruction::LH(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            let value = sign_extend(memory.read_halfword(target), 16);
            register_file.write(rdindex, value);
        }
        Instruction::LW(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            let value = memory.read_word(target);
            register_file.write(rdindex, value);
        }
        Instruction::LBU(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            let value = memory.read_byte(target);
            register_file.write(rdindex, value);
        }
        Instruction::LHU(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            let value = memory.read_halfword(target);
            register_file.write(rdindex, value);
        }
        Instruction::SB(rs1index, rs2index, simmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(simmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            memory.write_byte(target, _rs2);
        }
        Instruction::SH(rs1index, rs2index, simmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(simmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            memory.write_halfword(target, _rs2);
        }
        Instruction::SW(rs1index, rs2index, simmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let sign_imm = sign_extend(simmediate, 12) as i32;
            let target = add_signed!(_rs1, sign_imm) as usize;
            //println!("{:}, {:}, {:}", _rs1, _rs2, sign_imm);
            memory.write_word(target, _rs2);
        }
        Instruction::ADDI(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12) as i32;
            //println!("{:b}, {:b}, {:}", iimmediate, sign_imm, sign_imm);
            register_file.write(rdindex, add_signed!(_rs1, sign_imm));
        }
        Instruction::SLTI(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12);
            if (_rs1 as i32) < (sign_imm as i32) {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::SLTIU(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let sign_imm = sign_extend(iimmediate, 12);
            if _rs1 < sign_imm {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::XORI(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, _rs1 ^ sign_extend(iimmediate, 12));
        }
        Instruction::ORI(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, _rs1 | sign_extend(iimmediate, 12));
        }
        Instruction::ANDI(rdindex, rs1index, iimmediate) => {
            let _rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, _rs1 & sign_extend(iimmediate, 12));
        }
        Instruction::SLLI(rdindex, rs1index, iimmediate) => {
            let shamt = iimmediate & 0b1_1111;
            let _rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, _rs1 << shamt);
        }
        Instruction::SRLI(rdindex, rs1index, iimmediate) => {
            let shamt = iimmediate & 0b1_1111;
            let _rs1: RS1value = register_file.read(rs1index);
            register_file.write(rdindex, _rs1 >> shamt);
        }
        Instruction::SRAI(rdindex, rs1index, iimmediate) => {
            let shamt = iimmediate & 0b1_1111;
            let _rs1: RS1value = register_file.read(rs1index);
            let _value = sign_extend(_rs1 >> shamt, 32 - shamt);
            register_file.write(rdindex, _value);
        }
        Instruction::ADD(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 + _rs2)
        }
        Instruction::SUB(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 - _rs2)
        }
        Instruction::SLL(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 << (_rs2 & 0b1_1111))
        }
        Instruction::SLT(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            if (_rs1 as i32) < (_rs2 as i32) {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::SLTU(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            if _rs1 < _rs2 {
                register_file.write(rdindex, 1);
            } else {
                register_file.write(rdindex, 0);
            }
        }
        Instruction::XOR(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 ^ _rs2)
        }
        Instruction::SRL(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 >> (_rs2 & 0b1_1111))
        }
        Instruction::SRA(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            let shamt = _rs2 & 0b1_1111;
            register_file.write(rdindex, sign_extend(_rs1 >> shamt, 32 - shamt))
        }
        Instruction::OR(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 | _rs2)
        }
        Instruction::AND(rdindex, rs1index, rs2index) => {
            let _rs1: RS1value = register_file.read(rs1index);
            let _rs2: RS2value = register_file.read(rs2index);
            register_file.write(rdindex, _rs1 & _rs2)
        }
        Instruction::FENCE(_rdindex, _rs1index, _iimmediate) => { /* Nop */ }
        Instruction::ECALL() => {
            println!("ECALL (not implemented)");
        }
        Instruction::EBREAK() => {
            println!("EBREAK (not implemented)");
        }
        Instruction::CSRRW(_rd_index, _rs1, _i_imm) => {
        	register_file.write(_rd_index, register_file.csr.read(_i_imm));
        	register_file.csr.write(_i_imm, register_file.read(_rs1));
        }
		Instruction::CSRRS(_rd_index, _rs1, _i_imm) => {
			let _csr_value = register_file.csr.read(_i_imm);
			register_file.write(_rd_index, _csr_value);
			register_file.csr.write(_i_imm, register_file.read(_rs1) | _csr_value);
		}
		Instruction::CSRRC(_rd_index, _rs1, _i_imm) => {
			let _csr_value = register_file.csr.read(_i_imm);
			register_file.write(_rd_index, _csr_value);
			register_file.csr.write(_i_imm, !register_file.read(_rs1) & _csr_value);
		}
		Instruction::CSRRWI(_rd_index, _rs1, _i_imm) => {
			/* _rs1 is actual an immediate */
			let uimm = _rs1 as u32;
			register_file.write(_rd_index, register_file.csr.read(_i_imm));
        	register_file.csr.write(_i_imm, uimm);
		}
		Instruction::CSRRSI(_rd_index, _rs1, _i_imm) => {
			/* _rs1 is actual an immediate */
			let uimm = _rs1 as u32;
			let _csr_value = register_file.csr.read(_i_imm);
			register_file.write(_rd_index, _csr_value);
			register_file.csr.write(_i_imm, uimm | _csr_value);
		}
		Instruction::CSRRCI(_rd_index, _rs1, _i_imm) => {
			/* _rs1 is actual an immediate */
			let uimm = _rs1 as u32;
			let _csr_value = register_file.csr.read(_i_imm);
			register_file.write(_rd_index, _csr_value);
			register_file.csr.write(_i_imm, !uimm & _csr_value);
		}
		Instruction::MUL(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::MULH(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::MULHSU(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::MULHU(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::DIV(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::DIVU(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::REM(rdindex, rs1index, rs2index) => {
			todo!();
		}
		Instruction::REMU(rdindex, rs1index, rs2index) => {
			todo!();
		}
    }
    register_file.pc += 4;
}
