pub type Rindex = usize;
pub type RDindex = Rindex;
pub type RS1index = Rindex;
pub type RS2index = Rindex;

pub type RS1value = u32;
pub type RS2value = u32;

type Iimmediate = u32;
type Simmediate = u32;
type Bimmediate = u32;
type Uimmediate = u32;
type Jimmediate = u32;

type Funct3 = u32;
type Funct7 = u32;

fn immediate_i(instruction: u32) -> Iimmediate {
	return (instruction >> 20) as Iimmediate;
}

fn immediate_s(instruction: u32) -> Simmediate {
	return ((instruction >> 25) << 5) | ((instruction >> 7) & 0b1_1111) as Simmediate;
}

fn immediate_b(instruction: u32) -> Bimmediate {
	let _4_1 = (instruction >> 8) & 0b1111;
	let _10_5 = (instruction >> 25) & 0b11_1111;
	let _11 = (instruction >> 7) & 0b1;
	let _12 = (instruction >> 31) & 0b1;
	return ((_12 << 12) | (_11 << 11) | (_10_5 << 5) | (_4_1 << 1)) as Bimmediate;
}

fn immediate_u(instruction: u32) -> Uimmediate {
	return (instruction & 0b1111_1111_1111_1111_1111_0000_0000_0000) as Uimmediate;
}

fn immediate_j(instruction: u32) -> Jimmediate {
	let _10_1 = (instruction >> 21) & 0b11_1111_1111;
	let _11 = (instruction >> 20) & 0b1;
	let _19_12 = (instruction >> 12) & 0b1111_1111;
	let _20 = (instruction >> 31) & 0b1;
	return ((_20 << 20) | (_19_12 << 12) | (_11 << 11) | (_10_1 << 1)) as Jimmediate;
}

fn rs1(instruction: u32) -> RS1index {
	return ((instruction >> 15) & 0b1_1111) as RS1index;
}

fn rs2(instruction: u32) -> RS2index {
	return ((instruction >> 20) & 0b1_1111) as RS2index;
}

fn rd(instruction: u32) -> RDindex {
	return ((instruction >> 7) & 0b1_1111) as RDindex;
}

fn funct3(instruction: u32) -> Funct3 {
	return ((instruction >> 12) & 0b111) as Funct3
}

fn funct7(instruction: u32) -> Funct7 {
	return ((instruction >> 25) & 0b111_1111) as Funct7
}

macro_rules! isBaseInstructionSet {
	($inst:expr) => (($inst & 0b11) == 0b11)
}

macro_rules! OpUpperBits {
	($inst:expr) => (($inst >> 5) & 0b11)
}

macro_rules! OpLowerBits {
	($inst:expr) => (($inst >> 2) & 0b111)
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

#[derive(Debug)]
pub enum Instruction {
	LUI(RDindex, Uimmediate),
	AUIPC(RDindex, Uimmediate),
	JAL(RDindex, Jimmediate),
	JALR(RDindex, RS1index, Iimmediate),
	BEQ(RS1index, RS2index, Bimmediate),
	BNE(RS1index, RS2index, Bimmediate),
	BLT(RS1index, RS2index, Bimmediate),
	BGE(RS1index, RS2index, Bimmediate),
	BLTU(RS1index, RS2index, Bimmediate),
	BGEU(RS1index, RS2index, Bimmediate),
	LB(RDindex, RS1index, Iimmediate),
	LH(RDindex, RS1index, Iimmediate),
	LW(RDindex, RS1index, Iimmediate),
	LBU(RDindex, RS1index, Iimmediate),
	LHU(RDindex, RS1index, Iimmediate),
	SB(RS1index, RS2index, Simmediate),
	SH(RS1index, RS2index, Simmediate),
	SW(RS1index, RS2index, Simmediate),
	ADDI(RDindex, RS1index, Iimmediate),
	SLTI(RDindex, RS1index, Iimmediate),
	SLTIU(RDindex, RS1index, Iimmediate),
	XORI(RDindex, RS1index, Iimmediate),
	ORI(RDindex, RS1index, Iimmediate),
	ANDI(RDindex, RS1index, Iimmediate),
	SLLI(RDindex, RS1index, Iimmediate),
	SRLI(RDindex, RS1index, Iimmediate),
	SRAI(RDindex, RS1index, Iimmediate),
	ADD(RDindex, RS1index, RS2index),
	SUB(RDindex, RS1index, RS2index),
	SLL(RDindex, RS1index, RS2index),
	SLT(RDindex, RS1index, RS2index),
	SLTU(RDindex, RS1index, RS2index),
	XOR(RDindex, RS1index, RS2index),
	SRL(RDindex, RS1index, RS2index),
	SRA(RDindex, RS1index, RS2index),
	OR(RDindex, RS1index, RS2index),
	AND(RDindex, RS1index, RS2index),
	FENCE(RDindex, RS1index, Iimmediate),
	ECALL(),
	EBREAK(),
}

fn get_opcode(instruction: u32) -> OpCode {
	match OpUpperBits!(instruction) {
		0b00 => {
			match OpLowerBits!(instruction) {
				0b000 => return OpCode::LOAD,
				0b001 => return OpCode::LOADFP,
				0b010 => return OpCode::CUSTOM0,
				0b011 => return OpCode::MISCMEM,
				0b100 => return OpCode::OPIMM,
				0b101 => return OpCode::AUIPC,
				0b110 => return OpCode::OPIMM32,
				0b111 => return OpCode::LEN48,
		        _ => panic!("Shouldn't happen"),
			}
		},
		0b01 => {
			match OpLowerBits!(instruction) {
				0b000 => return OpCode::STORE,
				0b001 => return OpCode::STOREFP,
				0b010 => return OpCode::CUSTOM1,
				0b011 => return OpCode::AMO,
				0b100 => return OpCode::OP,
				0b101 => return OpCode::LUI,
				0b110 => return OpCode::OP32,
				0b111 => return OpCode::LEN64,
		        _ => panic!("Shouldn't happen"),
			}
		},
		0b10 => {
			match OpLowerBits!(instruction) {
				0b000 => return OpCode::MADD,
				0b001 => return OpCode::MSUB,
				0b010 => return OpCode::NMSUB,
				0b011 => return OpCode::NMADD,
				0b100 => return OpCode::OPFP,
				0b101 => return OpCode::RESERVED1,
				0b110 => return OpCode::CUSTOM2,
				0b111 => return OpCode::LEN482,
		        _ => panic!("Shouldn't happen"),
			}
		},
		0b11 => {
			match OpLowerBits!(instruction) {
				0b000 => return OpCode::BRANCH,
				0b001 => return OpCode::JALR,
				0b010 => return OpCode::RESERVED2,
				0b011 => return OpCode::JAL,
				0b100 => return OpCode::SYSTEM,
				0b101 => return OpCode::RESERVED3,
				0b110 => return OpCode::CUSTOM3,
				0b111 => return OpCode::LEN80,
		        _ => panic!("Shouldn't happen"),
			}
		},
		_ => panic!("Ahhhhhh: {:?}", OpUpperBits!(instruction)),
	}
}

pub fn decode(instruction: u32) -> Instruction {
	if !isBaseInstructionSet!(instruction) {
		panic!("Invalid base instruction type");
	}
	let op = get_opcode(instruction);
	match op {
		OpCode::LOAD => {
			/* All LOAD are I-Type instructions */
			let _rd_index: RDindex = rd(instruction);
			let _rs1: RS1index = rs1(instruction);
			let _i_imm: Iimmediate = immediate_i(instruction);
			match funct3(instruction){
				0b000 => {
					return Instruction::LB(_rd_index, _rs1, _i_imm);
				},
				0b001 => {
					return Instruction::LH(_rd_index, _rs1, _i_imm);
				},
				0b010 => {
					return Instruction::LW(_rd_index, _rs1, _i_imm);
				},
				0b100 => {
					return Instruction::LBU(_rd_index, _rs1, _i_imm);
				},
				0b101 => {
					return Instruction::LHU(_rd_index, _rs1, _i_imm);
				},
				_ => {
					panic!("Invalid funct3 I-Type");
				}
			}//
		},
		OpCode::LOADFP => todo!(),
		OpCode::CUSTOM0 => todo!(),
		OpCode::MISCMEM => {
			let _rd_index: RDindex = rd(instruction);
			let _rs1: RS1index = rs1(instruction);
			let _i_imm: Iimmediate = immediate_i(instruction);
			return Instruction::FENCE(_rd_index, _rs1, _i_imm);
		},
		OpCode::OPIMM => {
			/* All OPIMM are I-Type instructions */
			let _rd_index: RDindex = rd(instruction);
			let _rs1: RS1index = rs1(instruction);
			let _i_imm: Iimmediate = immediate_i(instruction);
			match funct3(instruction){
				0b000 => {
					return Instruction::ADDI(_rd_index, _rs1, _i_imm);
				},
				0b010 => {
					return Instruction::SLTI(_rd_index, _rs1, _i_imm);
				},
				0b011 => {
					return Instruction::SLTIU(_rd_index, _rs1, _i_imm);
				},
				0b100 => {
					return Instruction::XORI(_rd_index, _rs1, _i_imm);
				},
				0b110 => {
					return Instruction::ORI(_rd_index, _rs1, _i_imm);
				},
				0b111 => {
					return Instruction::ANDI(_rd_index, _rs1, _i_imm);
				},
				0b001 => {
					return Instruction::SLLI(_rd_index, _rs1, _i_imm);
				},
				0b101 => {
					if _i_imm == 0 {
						return Instruction::SRLI(_rd_index, _rs1, _i_imm);
					} else {
						return Instruction::SRAI(_rd_index, _rs1, _i_imm);
					}
				},
				_ => {
					panic!("Invalid funct3 I-Type");
				}
			}
		},
		OpCode::AUIPC => {
			/* U Type */
			let _rd_index: RDindex = rd(instruction);
			let _u_imm: Uimmediate = immediate_u(instruction);
			return Instruction::AUIPC(_rd_index, _u_imm);
		},
		OpCode::OPIMM32 => todo!(),
		OpCode::LEN48 => todo!(),
		OpCode::STORE => {
			/* STOREs are S-Type */
			let _rs1: RS1index = rs1(instruction);
			let _rs2: RS2index = rs2(instruction);
			let _s_imm: Simmediate = immediate_s(instruction);
			match funct3(instruction){
				0b000 => {
					return Instruction::SB(_rs1, _rs2, _s_imm);
				},
				0b001 => {
					return Instruction::SH(_rs1, _rs2, _s_imm);
				},
				0b010 => {
					return Instruction::SW(_rs1, _rs2, _s_imm);
				},
				_ => {
					panic!("Invalid funct3 S-Type");
				}
			}
		},
		OpCode::STOREFP => todo!(),
		OpCode::CUSTOM1 => todo!(),
		OpCode::AMO => todo!(),
		OpCode::OP => {
			/* All OP are R-Type instructions */
			let _rd_index: RDindex = rd(instruction);
			let _rs1: RS1index = rs1(instruction);
			let _rs2: RS2index = rs2(instruction);

			match funct3(instruction){
				0b000 => {
					if funct7(instruction) == 0 {
						return Instruction::ADD(_rd_index, _rs1, _rs2);
					} else {
						return Instruction::SUB(_rd_index, _rs1, _rs2);
					}
				},
				0b001 => {
					return Instruction::SLL(_rd_index, _rs1, _rs2);
				},
				0b010 => {
					return Instruction::SLT(_rd_index, _rs1, _rs2);
				},
				0b011 => {
					return Instruction::SLTU(_rd_index, _rs1, _rs2);
				},
				0b100 => {
					return Instruction::XOR(_rd_index, _rs1, _rs2);
				},
				0b101 => {
					if funct7(instruction) == 0 {
						return Instruction::SRL(_rd_index, _rs1, _rs2);
					} else {
						return Instruction::SRA(_rd_index, _rs1, _rs2);
					}
				},
				0b110 => {
					return Instruction::OR(_rd_index, _rs1, _rs2);
				},
				0b111 => {
					return Instruction::AND(_rd_index, _rs1, _rs2);
				},
				_ => {
					panic!("Invalid funct3 R-Type");
				}
			}
		},
		OpCode::LUI => {
			/* U Type */
			let _rd_index: RDindex = rd(instruction);
			let _u_imm: Uimmediate = immediate_u(instruction);
			return Instruction::LUI(_rd_index, _u_imm);
		},
		OpCode::OP32 => todo!(),
		OpCode::LEN64 => todo!(),
		OpCode::MADD => todo!(),
		OpCode::MSUB => todo!(),
		OpCode::NMSUB => todo!(),
		OpCode::NMADD => todo!(),
		OpCode::OPFP => todo!(),
		OpCode::RESERVED1 => todo!(),
		OpCode::CUSTOM2 => todo!(),
		OpCode::LEN482 => todo!(),
		OpCode::BRANCH => {
			/* B-Type instructions */
			let _rs1: RS1index = rs1(instruction);
			let _rs2: RS2index = rs2(instruction);
			let _b_imm: Bimmediate = immediate_b(instruction);
			match funct3(instruction){
				0b000 => {
					return Instruction::BEQ(_rs1, _rs2, _b_imm);
				}
				0b001 => {
					return Instruction::BNE(_rs1, _rs2, _b_imm);
				}
				0b100 => {
					return Instruction::BLT(_rs1, _rs2, _b_imm);
				}
				0b101 => {
					return Instruction::BGE(_rs1, _rs2, _b_imm);
				}
				0b110 => {
					return Instruction::BLTU(_rs1, _rs2, _b_imm);
				}
				0b111 => {
					return Instruction::BGEU(_rs1, _rs2, _b_imm);
				}
				_ => {
					panic!("Invalid funct3 B-Type");
				}
			}
		},
		OpCode::JALR => {
			//println!("JALR");
			/* I-Type instruction */
			let _rd_index: RDindex = rd(instruction);
			let _rs1: RS1index = rs1(instruction);
			let _i_imm: Iimmediate = immediate_i(instruction);
			return Instruction::JALR(_rd_index, _rs1, _i_imm);
		},
		OpCode::RESERVED2 => todo!(),
		OpCode::JAL => {
			//println!("JAL");
			let _rd_index: RDindex = rd(instruction);
			let _j_imm: Jimmediate = immediate_j(instruction);
			return Instruction::JAL(_rd_index, _j_imm);
		},
		OpCode::SYSTEM => {
			let _i_imm: Iimmediate = immediate_i(instruction);
			if _i_imm == 0 {
				return Instruction::ECALL();
			}
			return Instruction::EBREAK();
		},
		OpCode::RESERVED3 => todo!(),
		OpCode::CUSTOM3 => todo!(),
		OpCode::LEN80 => todo!(),
	}
}