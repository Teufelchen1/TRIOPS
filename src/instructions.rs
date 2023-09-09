use crate::decoder::{
    Immediate, RDindex, RS1index, RS2index,
};
use crate::system::register_name;

#[derive(Debug)]
pub enum Instruction {
    /* RV32I */
    LUI(RDindex, Immediate),
    AUIPC(RDindex, Immediate),
    JAL(RDindex, Immediate),
    JALR(RDindex, RS1index, Immediate),
    BEQ(RS1index, RS2index, Immediate),
    BNE(RS1index, RS2index, Immediate),
    BLT(RS1index, RS2index, Immediate),
    BGE(RS1index, RS2index, Immediate),
    BLTU(RS1index, RS2index, Immediate),
    BGEU(RS1index, RS2index, Immediate),
    LB(RDindex, RS1index, Immediate),
    LH(RDindex, RS1index, Immediate),
    LW(RDindex, RS1index, Immediate),
    LBU(RDindex, RS1index, Immediate),
    LHU(RDindex, RS1index, Immediate),
    SB(RS1index, RS2index, Immediate),
    SH(RS1index, RS2index, Immediate),
    SW(RS1index, RS2index, Immediate),
    ADDI(RDindex, RS1index, Immediate),
    SLTI(RDindex, RS1index, Immediate),
    SLTIU(RDindex, RS1index, Immediate),
    XORI(RDindex, RS1index, Immediate),
    ORI(RDindex, RS1index, Immediate),
    ANDI(RDindex, RS1index, Immediate),
    SLLI(RDindex, RS1index, Immediate),
    SRLI(RDindex, RS1index, Immediate),
    SRAI(RDindex, RS1index, Immediate),
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
    FENCE(RDindex, RS1index, Immediate),
    ECALL(),
    EBREAK(),
    MRET(),
    /* Zicsr */
    CSRRW(RDindex, RS1index, u32),
    CSRRS(RDindex, RS1index, u32),
    CSRRC(RDindex, RS1index, u32),
    CSRRWI(RDindex, RS1index, u32),
    CSRRSI(RDindex, RS1index, u32),
    CSRRCI(RDindex, RS1index, u32),
    /* M */
    MUL(RDindex, RS1index, RS2index),
    MULH(RDindex, RS1index, RS2index),
    MULHSU(RDindex, RS1index, RS2index),
    MULHU(RDindex, RS1index, RS2index),
    DIV(RDindex, RS1index, RS2index),
    DIVU(RDindex, RS1index, RS2index),
    REM(RDindex, RS1index, RS2index),
    REMU(RDindex, RS1index, RS2index),
    /* Compressed Q1 */
    CADDI4SPN(RDindex, Immediate),
    CFLD(RDindex, RS1index, Immediate),
    CLQ(RDindex, RS1index, Immediate),
    CLW(RDindex, RS1index, Immediate),
    CFLW(RDindex, RS1index, Immediate),
    CLD(RDindex, RS1index, Immediate),
    CFSD(RDindex, RS1index, Immediate),
    CSQ(RDindex, RS1index, Immediate),
    CSW(RDindex, RS1index, Immediate),
    CFSW(RDindex, RS1index, Immediate),
    CSD(RDindex, RS1index, Immediate),
    /* Compressed Q2 */
    CNOP(RDindex, Immediate),
    CADDI(RDindex, Immediate),
    CJAL(Immediate),
    CLI(RDindex, Immediate),
    CADDI16SP(RDindex, Immediate),
    CLUI(RDindex, Immediate),
    CSRLI(RDindex, Immediate),
    CSRAI(RDindex, Immediate),
    CANDI(RDindex, Immediate),
    CSUB(RDindex, RS2index),
    CXOR(RDindex, RS2index),
    COR(RDindex, RS2index),
    CAND(RDindex, RS2index),
    CJ(Immediate),
    CBEQZ(RS1index, Immediate),
    CBNEZ(RS1index, Immediate),
    /* Compressed Q3 */
    CSLLI(RDindex, Immediate),
    CFLDSP(RDindex, Immediate),
    CLWSP(RDindex, Immediate),
    CFLWSP(RDindex, Immediate),
    CJR(RS1index),
    CMV(RDindex, RS2index),
    CEBREAK(),
    CJALR(RS1index),
    CADD(RDindex, RS2index),
    CFSDSP(RS2index, Immediate),
    CSWSP(RS2index, Immediate),
    CFSWSP(RS2index, Immediate),
}

pub fn decompress(inst: &Instruction) -> Instruction {
    match *inst {
        Instruction::CADDI4SPN(rdindex, cnzuimmediate) => {
            Instruction::ADDI(rdindex, 2, cnzuimmediate)
        }
        Instruction::CFLD(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CLQ(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CLW(rdindex, rs1index, cuimmediate) => {
            Instruction::LW(rdindex, rs1index, cuimmediate)
        }
        Instruction::CFLW(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CLD(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CFSD(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CSQ(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CSW(rdindex, rs1index, cuimmediate) => {
            Instruction::SW(rdindex, rs1index, cuimmediate)
        }
        Instruction::CFSW(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CSD(rdindex, rs1index, cuimmediate) => todo!(),
        Instruction::CNOP(rdindex, cnzimmediate) => todo!(),
        Instruction::CADDI(rdindex, cnzimmediate) => {
            Instruction::ADDI(rdindex, rdindex, cnzimmediate)
        }
        Instruction::CJAL(cjimmediate) => Instruction::JAL(1, cjimmediate),
        Instruction::CLI(rdindex, cimmediate) => Instruction::ADDI(rdindex, 0, cimmediate),
        Instruction::CADDI16SP(rdindex, cnzimmediate) => Instruction::ADDI(2, 2, cnzimmediate),
        Instruction::CLUI(rdindex, cnzimmediate) => Instruction::LUI(rdindex, cnzimmediate),
        Instruction::CSRLI(rdindex, cnzuimmediate) => {
            Instruction::SRLI(rdindex, rdindex, cnzuimmediate)
        }
        Instruction::CSRAI(rdindex, cnzuimmediate) => {
            Instruction::SRAI(rdindex, rdindex, cnzuimmediate)
        }
        Instruction::CANDI(rdindex, cnzuimmediate) => {
            Instruction::ANDI(rdindex, rdindex, cnzuimmediate)
        }
        Instruction::CSUB(rdindex, rs2index) => Instruction::SUB(rdindex, rdindex, rs2index),
        Instruction::CXOR(rdindex, rs2index) => Instruction::XOR(rdindex, rdindex, rs2index),
        Instruction::COR(rdindex, rs2index) => Instruction::OR(rdindex, rdindex, rs2index),
        Instruction::CAND(rdindex, rs2index) => Instruction::AND(rdindex, rdindex, rs2index),
        Instruction::CJ(cjimmediate) => Instruction::JAL(0, cjimmediate),
        Instruction::CBEQZ(rs1index, cimmediate) => Instruction::BEQ(rs1index, 0, cimmediate),
        Instruction::CBNEZ(rs1index, cimmediate) => Instruction::BNE(rs1index, 0, cimmediate),
        Instruction::CSLLI(rdindex, cnzuimmediate) => {
            Instruction::SLLI(rdindex, rdindex, cnzuimmediate)
        }
        Instruction::CFLDSP(rdindex, cuimmediate) => todo!(),
        Instruction::CLWSP(rdindex, cuimmediate) => Instruction::LW(rdindex, 2, cuimmediate),
        Instruction::CFLWSP(rdindex, cuimmediate) => todo!(),
        Instruction::CJR(rs1index) => Instruction::JALR(0, rs1index, 0),
        Instruction::CMV(rdindex, rs2index) => Instruction::ADD(rdindex, 0, rs2index),
        Instruction::CEBREAK() => Instruction::EBREAK(),
        Instruction::CJALR(rs1index) => Instruction::JALR(1, rs1index, 0),
        Instruction::CADD(rdindex, rs2index) => Instruction::ADD(rdindex, rdindex, rs2index),
        Instruction::CFSDSP(rs2index, cluimmediate) => todo!(),
        Instruction::CSWSP(rs2index, cluimmediate) => Instruction::SW(2, rs2index, cluimmediate),
        Instruction::CFSWSP(rs2index, cluimmediate) => todo!(),
        _ => panic!(),
    }
}

impl Instruction {
    pub fn is_zicsr(&self) -> bool {
        matches!(
            self,
            Self::CSRRCI(..)
                | Self::CSRRW(..)
                | Self::CSRRS(..)
                | Self::CSRRC(..)
                | Self::CSRRWI(..)
                | Self::CSRRSI(..)
        )
    }
    pub fn is_m(&self) -> bool {
        matches!(
            self,
            Self::MUL(..)
                | Self::MULH(..)
                | Self::MULHSU(..)
                | Self::MULHU(..)
                | Self::DIV(..)
                | Self::DIVU(..)
                | Self::REM(..)
                | Self::REMU(..)
        )
    }
    pub fn is_compressed(&self) -> bool {
        matches!(
            self,
            Self::CADDI4SPN(..)
                | Self::CFLD(..)
                | Self::CLQ(..)
                | Self::CLW(..)
                | Self::CFLW(..)
                | Self::CLD(..)
                | Self::CFSD(..)
                | Self::CSQ(..)
                | Self::CSW(..)
                | Self::CFSW(..)
                | Self::CSD(..)
                | Self::CNOP(..)
                | Self::CADDI(..)
                | Self::CJAL(..)
                | Self::CLI(..)
                | Self::CADDI16SP(..)
                | Self::CLUI(..)
                | Self::CSRLI(..)
                | Self::CSRAI(..)
                | Self::CANDI(..)
                | Self::CSUB(..)
                | Self::CXOR(..)
                | Self::COR(..)
                | Self::CAND(..)
                | Self::CJ(..)
                | Self::CBEQZ(..)
                | Self::CBNEZ(..)
                | Self::CSLLI(..)
                | Self::CFLDSP(..)
                | Self::CLWSP(..)
                | Self::CFLWSP(..)
                | Self::CJR(..)
                | Self::CMV(..)
                | Self::CEBREAK()
                | Self::CJALR(..)
                | Self::CADD(..)
                | Self::CFSDSP(..)
                | Self::CSWSP(..)
                | Self::CFSWSP(..)
        )
    }
    #[allow(clippy::too_many_lines)]
    pub fn print(&self) -> String {
        match *self {
            /* RV32I */
            Instruction::LUI(rdindex, uimmediate) => {
                format!("lui {:}, {:}", register_name(rdindex), uimmediate)
            }
            Instruction::AUIPC(rdindex, uimmediate) => {
                format!("auipc {:}, {:}", register_name(rdindex), uimmediate)
            }
            Instruction::JAL(rdindex, jimmediate) => {
                format!("jal {:}, {:}", register_name(rdindex), jimmediate)
            }
            Instruction::JALR(rdindex, rs1index, iimmediate) => format!(
                "jalr {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::BEQ(rs1index, rs2index, bimmediate) => format!(
                "beq {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                bimmediate
            ),
            Instruction::BNE(rs1index, rs2index, bimmediate) => format!(
                "bne {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                bimmediate
            ),
            Instruction::BLT(rs1index, rs2index, bimmediate) => format!(
                "blt {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                bimmediate
            ),
            Instruction::BGE(rs1index, rs2index, bimmediate) => format!(
                "bge {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                bimmediate
            ),
            Instruction::BLTU(rs1index, rs2index, bimmediate) => format!(
                "bltu {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                bimmediate
            ),
            Instruction::BGEU(rs1index, rs2index, bimmediate) => format!(
                "bgeu {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                bimmediate
            ),
            Instruction::LB(rdindex, rs1index, iimmediate) => format!(
                "lb {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::LH(rdindex, rs1index, iimmediate) => format!(
                "lh {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::LW(rdindex, rs1index, iimmediate) => format!(
                "lw {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::LBU(rdindex, rs1index, iimmediate) => format!(
                "lbu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::LHU(rdindex, rs1index, iimmediate) => format!(
                "lhu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::SB(rs1index, rs2index, simmediate) => format!(
                "sb {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                simmediate,
            ),
            Instruction::SH(rs1index, rs2index, simmediate) => format!(
                "sh {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                simmediate,
            ),
            Instruction::SW(rs1index, rs2index, simmediate) => format!(
                "sw {:}, {:}, {:}",
                register_name(rs1index),
                register_name(rs2index),
                simmediate,
            ),
            Instruction::ADDI(rdindex, rs1index, iimmediate) => format!(
                "addi {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::SLTI(rdindex, rs1index, iimmediate) => format!(
                "slti {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::SLTIU(rdindex, rs1index, iimmediate) => format!(
                "sltiu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::XORI(rdindex, rs1index, iimmediate) => format!(
                "xori {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::ORI(rdindex, rs1index, iimmediate) => format!(
                "ori {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::ANDI(rdindex, rs1index, iimmediate) => format!(
                "andi {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::SLLI(rdindex, rs1index, iimmediate) => format!(
                "slli {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::SRLI(rdindex, rs1index, iimmediate) => format!(
                "srli {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::SRAI(rdindex, rs1index, iimmediate) => format!(
                "srai {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::ADD(rdindex, rs1index, rs2index) => format!(
                "add {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::SUB(rdindex, rs1index, rs2index) => format!(
                "sub {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::SLL(rdindex, rs1index, rs2index) => format!(
                "sll {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::SLT(rdindex, rs1index, rs2index) => format!(
                "slt {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::SLTU(rdindex, rs1index, rs2index) => format!(
                "sltu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::XOR(rdindex, rs1index, rs2index) => format!(
                "xor {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::SRL(rdindex, rs1index, rs2index) => format!(
                "srl {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::SRA(rdindex, rs1index, rs2index) => format!(
                "sra {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::OR(rdindex, rs1index, rs2index) => format!(
                "or {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::AND(rdindex, rs1index, rs2index) => format!(
                "and {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index),
            ),
            Instruction::FENCE(rdindex, rs1index, iimmediate) => format!(
                "fence {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::ECALL() => "ecall".to_string(),
            Instruction::EBREAK() => "ebreak".to_string(),
            Instruction::MRET() => "mret".to_string(),
            /* Zicsr */
            Instruction::CSRRW(rdindex, rs1index, iimmediate) => format!(
                "csrrw {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRS(rdindex, rs1index, iimmediate) => format!(
                "csrrs {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRC(rdindex, rs1index, iimmediate) => format!(
                "csrrc {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRWI(rdindex, rs1index, iimmediate) => format!(
                "csrrwi {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRSI(rdindex, rs1index, iimmediate) => format!(
                "csrrsi {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRCI(rdindex, rs1index, iimmediate) => format!(
                "csrrci {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                iimmediate
            ),
            /* M */
            Instruction::MUL(rdindex, rs1index, rs2index) => format!(
                "mul {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::MULH(rdindex, rs1index, rs2index) => format!(
                "mulh {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::MULHSU(rdindex, rs1index, rs2index) => format!(
                "mulhsu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::MULHU(rdindex, rs1index, rs2index) => format!(
                "mulhu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::DIV(rdindex, rs1index, rs2index) => format!(
                "div {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::DIVU(rdindex, rs1index, rs2index) => format!(
                "divu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::REM(rdindex, rs1index, rs2index) => format!(
                "rem {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::REMU(rdindex, rs1index, rs2index) => format!(
                "remu {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                register_name(rs2index)
            ),
            Instruction::CADDI4SPN(rdindex, cnzuimmediate) => {
                format!("C.ADDI4SPN {:}, {:}", register_name(rdindex), cnzuimmediate)
            }
            Instruction::CFLD(rdindex, rs1index, cuimmediate) => format!(
                "C.FLD {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CLQ(rdindex, rs1index, cuimmediate) => format!(
                "C.LQ {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CLW(rdindex, rs1index, cuimmediate) => format!(
                "C.LW {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CFLW(rdindex, rs1index, cuimmediate) => format!(
                "C.FLW {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CLD(rdindex, rs1index, cuimmediate) => format!(
                "C.LD {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CFSD(rdindex, rs1index, cuimmediate) => format!(
                "C.FSD {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CSQ(rdindex, rs1index, cuimmediate) => format!(
                "C.SQ {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CSW(rdindex, rs1index, cuimmediate) => format!(
                "C.SW {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CFSW(rdindex, rs1index, cuimmediate) => format!(
                "C.FSW {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CSD(rdindex, rs1index, cuimmediate) => format!(
                "C.SD {:}, {:}, {:}",
                register_name(rdindex),
                register_name(rs1index),
                cuimmediate
            ),
            Instruction::CNOP(rdindex, cnzimmediate) => {
                format!("C.NOP {:}, {:}", register_name(rdindex), cnzimmediate)
            }
            Instruction::CADDI(rdindex, cnzimmediate) => {
                format!("C.ADDI {:}, {:}", register_name(rdindex), cnzimmediate)
            }
            Instruction::CJAL(cjimmediate) => format!("C.JAL {cjimmediate:}"),
            Instruction::CLI(rdindex, cimmediate) => {
                format!("C.LI {:}, {:}", register_name(rdindex), cimmediate)
            }
            Instruction::CADDI16SP(rdindex, cnzimmediate) => {
                format!("C.ADDI16SP {:}, {:}", register_name(rdindex), cnzimmediate)
            }
            Instruction::CLUI(rdindex, cnzimmediate) => {
                format!("C.LUI {:}, {:}", register_name(rdindex), cnzimmediate)
            }
            Instruction::CSRLI(rdindex, cnzuimmediate) => {
                format!("C.SRLI {:}, {:}", register_name(rdindex), cnzuimmediate)
            }
            Instruction::CSRAI(rdindex, cnzuimmediate) => {
                format!("C.SRAI {:}, {:}", register_name(rdindex), cnzuimmediate)
            }
            Instruction::CANDI(rdindex, cnzuimmediate) => {
                format!("C.ANDI {:}, {:}", register_name(rdindex), cnzuimmediate)
            }
            Instruction::CSUB(rdindex, rs2index) => format!(
                "C.SUB {:}, {:}",
                register_name(rdindex),
                register_name(rs2index)
            ),
            Instruction::CXOR(rdindex, rs2index) => format!(
                "C.XOR {:}, {:}",
                register_name(rdindex),
                register_name(rs2index)
            ),
            Instruction::COR(rdindex, rs2index) => format!(
                "C.OR {:}, {:}",
                register_name(rdindex),
                register_name(rs2index)
            ),
            Instruction::CAND(rdindex, rs2index) => format!(
                "C.AND {:}, {:}",
                register_name(rdindex),
                register_name(rs2index)
            ),
            Instruction::CJ(cjimmediate) => format!("C.J {cjimmediate:}"),
            Instruction::CBEQZ(rs1index, cimmediate) => {
                format!("C.BEQZ {:}, {:}", register_name(rs1index), cimmediate)
            }
            Instruction::CBNEZ(rs1index, cimmediate) => {
                format!("C.BNEZ {:}, {:}", register_name(rs1index), cimmediate)
            }
            Instruction::CSLLI(rdindex, cnzuimmediate) => {
                format!("C.SLLI {:}, {:}", register_name(rdindex), cnzuimmediate)
            }
            Instruction::CFLDSP(rdindex, cuimmediate) => {
                format!("C.FLDSP {:}, {:}", register_name(rdindex), cuimmediate)
            }
            Instruction::CLWSP(rdindex, cuimmediate) => {
                format!("C.LWSP {:}, {:}", register_name(rdindex), cuimmediate)
            }
            Instruction::CFLWSP(rdindex, cuimmediate) => {
                format!("C.FLWSP {:}, {:}", register_name(rdindex), cuimmediate)
            }
            Instruction::CJR(rs1index) => format!("C.JR {:}", register_name(rs1index)),
            Instruction::CMV(rdindex, rs2index) => format!(
                "C.MV {:}, {:}",
                register_name(rdindex),
                register_name(rs2index)
            ),
            Instruction::CEBREAK() => "C.EBREAK".to_string(),
            Instruction::CJALR(rs1index) => {
                format!("C.JALR {:}", register_name(rs1index))
            }
            Instruction::CADD(rdindex, rs2index) => format!(
                "C.ADD {:}, {:}",
                register_name(rdindex),
                register_name(rs2index)
            ),
            Instruction::CFSDSP(rs2index, cluimmediate) => {
                format!("C.FSDSP {:}, {:}", register_name(rs2index), cluimmediate)
            }
            Instruction::CSWSP(rs2index, cluimmediate) => {
                format!("C.SWSP {:}, {:}", register_name(rs2index), cluimmediate)
            }
            Instruction::CFSWSP(rs2index, cluimmediate) => {
                format!("C.FSWSP {:}, {:}", register_name(rs2index), cluimmediate)
            }
            _ => format!("{self:?}"),
        }
    }
}
