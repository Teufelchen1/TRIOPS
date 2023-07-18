#[derive(Debug)]
enum OpCode {
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

type RDindex u32;
type RS1 u32;
type RS2 u32;

enum Instruction {
	ADD(RDindex, RS1, RS2),
	SUB(RDindex, RS1, RS2),
	AUIPC(u32, u32),
}

#[derive(Default)]
struct RegisterFile {
	regs: [u32; 32],
	pc: u32,
}

impl RegisterFile {
	fn read(&self, index: u32) -> u32 {
		return self.regs[index];
	}

	fn write(&mut self, index: u32, value: u32) {
		if index > 0 {
			self.regs[index] = value;
		}
	}
}

fn sign_extend(num: u32, bitnum: u32) -> u32 {
	let msb = num >> bitnum;
	let sign = 0 - msb;
	let sign_filled = sign << bitnum;
	return sign_filled | num;
}

macro_rules! immediate110 {
	($inst:expr) => ($inst >> 20)
}

macro_rules! immediate3112 {
	($inst:expr) => ($inst & 0b1111_1111_1111_1111_1111_0000_0000_0000)
}

macro_rules! rs1 {
	($inst:expr) => (($inst >> 15) & 0b1_1111)
}

macro_rules! rs2 {
	($inst:expr) => (($inst >> 20) & 0b1_1111)
}

macro_rules! rd {
	($inst:expr) => (($inst >> 7) & 0b1_1111)
}

macro_rules! funct3 {
	($inst:expr) => (($inst >> 12) & 0b111)
}

macro_rules! funct7 {
	($inst:expr) => (($inst >> 25) & 0b111_1111)
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

fn get_opcode(instruction: u32) -> OpCode {
	match OpUpperBits!(instruction) {
		0b00 => {
			println!("00");
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
			println!("01");
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
			println!("10");
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
			println!("11");
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

fn step(register_file: RegisterFile, instruction: u32) {
	if !isBaseInstructionSet!(instruction) {
		panic!("Invalid base instruction type");
	}
	let op = get_opcode(instruction);
	println!("Opcode: {:?}", op);
	match op {
		OpCode::LOAD => {
			println!("I-Type");
			match funct3!(instruction){
				0b000 => {
					println!("LB");
				},
				0b001 => {
					println!("LH");
				},
				0b010 => {
					println!("LW");
				},
				0b100 => {
					println!("LBU");
				},
				0b101 => {
					println!("LHU");
				},
				_ => {
					panic!("Invalid funct3 I-Type");
				}
			}
		},
		OpCode::LOADFP => panic!("Not implemented."),
		OpCode::CUSTOM0 => panic!("Not implemented."),
		OpCode::MISCMEM => panic!("Not implemented."),
		OpCode::OPIMM => {
			println!("I-Type");
			let _rd_index = rd!(instruction);
			let _rs1 = register_file.read(rs1(instruction));
			let _immediate = immediate110!(instruction);
			match funct3!(instruction){
				0b000 => {
					println!("ADDI");
					register_file.write(_rd_index, _rs1 + sign_extend(_immediate, 12));
				},
				0b010 => {
					println!("SLTI");
					if _rs1 as i32 < sign_extend(_immediate, 12) as i32{
						register_file.write(_rd_index, 1);
					} else {
						register_file.write(_rd_index, 0);
					}
				},
				0b011 => {
					println!("SLTIU");
					if _rs1 < sign_extend(_immediate, 12) {
						register_file.write(_rd_index, 1);
					} else {
						register_file.write(_rd_index, 0);
					}
				},
				0b100 => {
					println!("XORI");
					register_file.write(_rd_index, _rs1 ^ sign_extend(_immediate, 12));
				},
				0b110 => {
					println!("ORI");
					register_file.write(_rd_index, _rs1 | sign_extend(_immediate, 12));
				},
				0b111 => {
					println!("ANDI");
					register_file.write(_rd_index, _rs1 & sign_extend(_immediate, 12));
				},
				0b001 => {
					println!("SLLI");
					let shift_amount = _immediate & 0b1_1111;
					register_file.write(_rd_index, _rs1 << shift_amount);
				},
				0b101 => {
					let shift_amount = _immediate & 0b1_1111;
					if _immediate & 0b010_0000 {
						println!("SRAI");
						register_file.write(_rd_index, _rs1 >> shift_amount);
					} else {
						println!("SRLI");
						register_file.write(_rd_index, sign_extend(_rs1 >> shift_amount, 32-shift_amount));
					}
				},
				_ => {
					panic!("Invalid funct3 I-Type");
				}
			}
		},
		OpCode::AUIPC => {
			println!("AUIPC");
			let _immediate = immediate3112!(instruction);
			let _rd_index = rd!(instruction);
			register_file.write(_rd_index, register_file.pc + _immediate);
		},
		OpCode::OPIMM32 => panic!("Not implemented."),
		OpCode::LEN48 => panic!("Not implemented."),
		OpCode::STORE => {
			match funct3!(instruction){
				0b000 => {
					println!("SB");
				},
				0b001 => {
					println!("SH");
				},
				0b010 => {
					println!("SW");
				},
				_ => {
					panic!("Invalid funct3 I-Type");
				}
			}
		},
		OpCode::STOREFP => panic!("Not implemented."),
		OpCode::CUSTOM1 => panic!("Not implemented."),
		OpCode::AMO => panic!("Not implemented."),
		OpCode::OP => {
			let _rd_index = rd!(instruction);
			let _rs1 = register_file.read(rs1!(instruction));
			let _rs2 = register_file.read(rs2!(instruction));
			let _funct7 = funct7!(instruction);
			match funct3!(instruction){
				0b000 => {
					if _funct7 == 0 {
						println!("ADD");
						return Instruction::ADD(_rd_index, _rs1, _rs2);
						register_file.write(_rd_index, _rs1 + _rs2);
					} else {
						println!("SUB");
						register_file.write(_rd_index, _rs1 + _rs2);
					}
				},
				0b001 => {
					println!("SLL");
				},
				0b010 => {
					println!("SLT");
				},
				0b011 => {
					println!("SLTU");
				},
				0b100 => {
					println!("XOR");
				},
				0b101 => {
					println!("SRL");
					println!("SRA");
				},
				0b110 => {
					println!("OR");
				},
				0b111 => {
					println!("AND");
				},
				_ => {
					panic!("Invalid funct3 R-Type");
				}
			}
		},
		OpCode::LUI => {
			println!("LUI");
			let _immediate = immediate3112!(instruction);
			let _rd_index = rd!(instruction);
			register_file.write(_rd_index, _immediate);
		},
		OpCode::OP32 => panic!("Not implemented."),
		OpCode::LEN64 => panic!("Not implemented."),
		OpCode::MADD => panic!("Not implemented."),
		OpCode::MSUB => panic!("Not implemented."),
		OpCode::NMSUB => panic!("Not implemented."),
		OpCode::NMADD => panic!("Not implemented."),
		OpCode::OPFP => panic!("Not implemented."),
		OpCode::RESERVED1 => panic!("Not implemented."),
		OpCode::CUSTOM2 => panic!("Not implemented."),
		OpCode::LEN482 => panic!("Not implemented."),
		OpCode::BRANCH => {
			match funct3!(instruction){
				0b000 => {
					println!("BEQ");
				}
				0b001 => {
					println!("BNE");
				}
				0b100 => {
					println!("BLT");
				}
				0b101 => {
					println!("BGE");
				}
				0b110 => {
					println!("BLTU");
				}
				0b111 => {
					println!("BGEU");
				}
			}
		},
		OpCode::JALR => {
			println!("JALR");
		},
		OpCode::RESERVED2 => panic!("Not implemented."),
		OpCode::JAL => {
			println!("JAL");
		},
		OpCode::SYSTEM => panic!("Not implemented."),
		OpCode::RESERVED3 => panic!("Not implemented."),
		OpCode::CUSTOM3 => panic!("Not implemented."),
		OpCode::LEN80 => panic!("Not implemented."),
	}
}

fn exec(ins: Instruction) {
	match ins {
		Instruction::ADD(rd, rs1, rs2) => {
			register_file.write(rd, rs1 + rs2)
		}
	}
}

fn main() {
	let instruction: u32 = 0x5fff1197;
	let mut register_file: RegisterFile = RegisterFile::default();
	step(register_file, instruction);
}