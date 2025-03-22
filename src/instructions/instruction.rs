//! Here lies the definition of all available risc-v instructions.
//! The scope is defined as:
//!  - All business related to `Instructions`
//!  - Define all `Instruction` enums
//!  - Host the decompression of `Instruction` via `decompress()`
//!  - Provide classification via `is_ziscr()`, `is_m()` and `is_compressed()`
//!  - Pretty print `Instruction`
use crate::cpu::index_to_name;
use crate::instructions::{Immediate, RDindex, RS1index, RS2index};

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
    /* A */
    LRW(RDindex, RS1index),
    SCW(RDindex, RS1index, RS2index),
    AMOSWAPW(RDindex, RS1index, RS2index),
    AMOADDW(RDindex, RS1index, RS2index),
    AMOXORW(RDindex, RS1index, RS2index),
    AMOANDW(RDindex, RS1index, RS2index),
    AMOORW(RDindex, RS1index, RS2index),
    AMOMINW(RDindex, RS1index, RS2index),
    AMOMAXW(RDindex, RS1index, RS2index),
    AMOMINUW(RDindex, RS1index, RS2index),
    AMOMAXUW(RDindex, RS1index, RS2index),
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
    /* Priv */
    WFI(),
}

impl Instruction {
    pub fn decompress(&self) -> Instruction {
        match *self {
            Instruction::CADDI4SPN(rdindex, cnzuimmediate) => {
                Instruction::ADDI(rdindex, 2, cnzuimmediate)
            }
            Instruction::CFLD(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CLQ(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CLW(rdindex, rs1index, cuimmediate) => {
                Instruction::LW(rdindex, rs1index, cuimmediate)
            }
            Instruction::CFLW(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CLD(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CFSD(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CSQ(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CSW(rdindex, rs1index, cuimmediate) => {
                Instruction::SW(rdindex, rs1index, cuimmediate)
            }
            Instruction::CFSW(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CSD(_rdindex, _rs1index, _cuimmediate) => todo!(),
            Instruction::CNOP(_rdindex, _cnzimmediate) => todo!(),
            Instruction::CADDI(rdindex, cnzimmediate) => {
                Instruction::ADDI(rdindex, rdindex, cnzimmediate)
            }
            Instruction::CJAL(cjimmediate) => Instruction::JAL(1, cjimmediate),
            Instruction::CLI(rdindex, cimmediate) => Instruction::ADDI(rdindex, 0, cimmediate),
            Instruction::CADDI16SP(_rdindex, cnzimmediate) => Instruction::ADDI(2, 2, cnzimmediate),
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
            Instruction::CFLDSP(_rdindex, _cuimmediate) => todo!(),
            Instruction::CLWSP(rdindex, cuimmediate) => Instruction::LW(rdindex, 2, cuimmediate),
            Instruction::CFLWSP(_rdindex, _cuimmediate) => todo!(),
            Instruction::CJR(rs1index) => Instruction::JALR(0, rs1index, 0),
            Instruction::CMV(rdindex, rs2index) => Instruction::ADD(rdindex, 0, rs2index),
            Instruction::CEBREAK() => Instruction::EBREAK(),
            Instruction::CJALR(rs1index) => Instruction::JALR(1, rs1index, 0),
            Instruction::CADD(rdindex, rs2index) => Instruction::ADD(rdindex, rdindex, rs2index),
            Instruction::CFSDSP(_rs2index, _cluimmediate) => todo!(),
            Instruction::CSWSP(rs2index, cluimmediate) => {
                Instruction::SW(2, rs2index, cluimmediate)
            }
            Instruction::CFSWSP(_rs2index, _cluimmediate) => todo!(),
            _ => panic!(),
        }
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
                format!("lui {:}, {:}", index_to_name(rdindex), uimmediate)
            }
            Instruction::AUIPC(rdindex, uimmediate) => {
                format!("auipc {:}, {:}", index_to_name(rdindex), uimmediate)
            }
            Instruction::JAL(rdindex, jimmediate) => {
                format!("jal {:}, {:}", index_to_name(rdindex), jimmediate)
            }
            Instruction::JALR(rdindex, rs1index, iimmediate) => format!(
                "jalr {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::BEQ(rs1index, rs2index, bimmediate) => format!(
                "beq {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                bimmediate
            ),
            Instruction::BNE(rs1index, rs2index, bimmediate) => format!(
                "bne {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                bimmediate
            ),
            Instruction::BLT(rs1index, rs2index, bimmediate) => format!(
                "blt {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                bimmediate
            ),
            Instruction::BGE(rs1index, rs2index, bimmediate) => format!(
                "bge {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                bimmediate
            ),
            Instruction::BLTU(rs1index, rs2index, bimmediate) => format!(
                "bltu {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                bimmediate
            ),
            Instruction::BGEU(rs1index, rs2index, bimmediate) => format!(
                "bgeu {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                bimmediate
            ),
            Instruction::LB(rdindex, rs1index, iimmediate) => format!(
                "lb {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::LH(rdindex, rs1index, iimmediate) => format!(
                "lh {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::LW(rdindex, rs1index, iimmediate) => format!(
                "lw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::LBU(rdindex, rs1index, iimmediate) => format!(
                "lbu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::LHU(rdindex, rs1index, iimmediate) => format!(
                "lhu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::SB(rs1index, rs2index, simmediate) => format!(
                "sb {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                simmediate,
            ),
            Instruction::SH(rs1index, rs2index, simmediate) => format!(
                "sh {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                simmediate,
            ),
            Instruction::SW(rs1index, rs2index, simmediate) => format!(
                "sw {:}, {:}, {:}",
                index_to_name(rs1index),
                index_to_name(rs2index),
                simmediate,
            ),
            Instruction::ADDI(rdindex, rs1index, iimmediate) => format!(
                "addi {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::SLTI(rdindex, rs1index, iimmediate) => format!(
                "slti {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::SLTIU(rdindex, rs1index, iimmediate) => format!(
                "sltiu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::XORI(rdindex, rs1index, iimmediate) => format!(
                "xori {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::ORI(rdindex, rs1index, iimmediate) => format!(
                "ori {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::ANDI(rdindex, rs1index, iimmediate) => format!(
                "andi {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::SLLI(rdindex, rs1index, iimmediate) => format!(
                "slli {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::SRLI(rdindex, rs1index, iimmediate) => format!(
                "srli {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::SRAI(rdindex, rs1index, iimmediate) => format!(
                "srai {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::ADD(rdindex, rs1index, rs2index) => format!(
                "add {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::SUB(rdindex, rs1index, rs2index) => format!(
                "sub {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::SLL(rdindex, rs1index, rs2index) => format!(
                "sll {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::SLT(rdindex, rs1index, rs2index) => format!(
                "slt {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::SLTU(rdindex, rs1index, rs2index) => format!(
                "sltu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::XOR(rdindex, rs1index, rs2index) => format!(
                "xor {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::SRL(rdindex, rs1index, rs2index) => format!(
                "srl {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::SRA(rdindex, rs1index, rs2index) => format!(
                "sra {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::OR(rdindex, rs1index, rs2index) => format!(
                "or {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::AND(rdindex, rs1index, rs2index) => format!(
                "and {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index),
            ),
            Instruction::FENCE(rdindex, rs1index, iimmediate) => format!(
                "fence {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::ECALL() => "ecall".to_string(),
            Instruction::EBREAK() => "ebreak".to_string(),
            Instruction::MRET() => "mret".to_string(),
            /* Zicsr */
            Instruction::CSRRW(rdindex, rs1index, iimmediate) => format!(
                "csrrw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRS(rdindex, rs1index, iimmediate) => format!(
                "csrrs {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRC(rdindex, rs1index, iimmediate) => format!(
                "csrrc {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRWI(rdindex, rs1index, iimmediate) => format!(
                "csrrwi {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRSI(rdindex, rs1index, iimmediate) => format!(
                "csrrsi {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            Instruction::CSRRCI(rdindex, rs1index, iimmediate) => format!(
                "csrrci {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                iimmediate
            ),
            /* M */
            Instruction::MUL(rdindex, rs1index, rs2index) => format!(
                "mul {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::MULH(rdindex, rs1index, rs2index) => format!(
                "mulh {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::MULHSU(rdindex, rs1index, rs2index) => format!(
                "mulhsu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::MULHU(rdindex, rs1index, rs2index) => format!(
                "mulhu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::DIV(rdindex, rs1index, rs2index) => format!(
                "div {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::DIVU(rdindex, rs1index, rs2index) => format!(
                "divu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::REM(rdindex, rs1index, rs2index) => format!(
                "rem {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::REMU(rdindex, rs1index, rs2index) => format!(
                "remu {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::LRW(rdindex, rs1index) => format!(
                "lrw {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index)
            ),
            Instruction::SCW(rdindex, rs1index, rs2index) => format!(
                "scw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOSWAPW(rdindex, rs1index, rs2index) => format!(
                "amoswapw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOADDW(rdindex, rs1index, rs2index) => format!(
                "amoaddw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOXORW(rdindex, rs1index, rs2index) => format!(
                "amoxorw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOANDW(rdindex, rs1index, rs2index) => format!(
                "amoandw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOORW(rdindex, rs1index, rs2index) => format!(
                "amoorw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOMINW(rdindex, rs1index, rs2index) => format!(
                "amominw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOMAXW(rdindex, rs1index, rs2index) => format!(
                "amomaxw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOMINUW(rdindex, rs1index, rs2index) => format!(
                "amominuw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::AMOMAXUW(rdindex, rs1index, rs2index) => format!(
                "amomaxuw {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                index_to_name(rs2index)
            ),
            Instruction::CADDI4SPN(rdindex, cnzuimmediate) => {
                format!("C.ADDI4SPN {:}, {:}", index_to_name(rdindex), cnzuimmediate)
            }
            Instruction::CFLD(rdindex, rs1index, cuimmediate) => format!(
                "C.FLD {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CLQ(rdindex, rs1index, cuimmediate) => format!(
                "C.LQ {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CLW(rdindex, rs1index, cuimmediate) => format!(
                "C.LW {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CFLW(rdindex, rs1index, cuimmediate) => format!(
                "C.FLW {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CLD(rdindex, rs1index, cuimmediate) => format!(
                "C.LD {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CFSD(rdindex, rs1index, cuimmediate) => format!(
                "C.FSD {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CSQ(rdindex, rs1index, cuimmediate) => format!(
                "C.SQ {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CSW(rdindex, rs1index, cuimmediate) => format!(
                "C.SW {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CFSW(rdindex, rs1index, cuimmediate) => format!(
                "C.FSW {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CSD(rdindex, rs1index, cuimmediate) => format!(
                "C.SD {:}, {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs1index),
                cuimmediate
            ),
            Instruction::CNOP(rdindex, cnzimmediate) => {
                format!("C.NOP {:}, {:}", index_to_name(rdindex), cnzimmediate)
            }
            Instruction::CADDI(rdindex, cnzimmediate) => {
                format!("C.ADDI {:}, {:}", index_to_name(rdindex), cnzimmediate)
            }
            Instruction::CJAL(cjimmediate) => format!("C.JAL {cjimmediate:}"),
            Instruction::CLI(rdindex, cimmediate) => {
                format!("C.LI {:}, {:}", index_to_name(rdindex), cimmediate)
            }
            Instruction::CADDI16SP(rdindex, cnzimmediate) => {
                format!("C.ADDI16SP {:}, {:}", index_to_name(rdindex), cnzimmediate)
            }
            Instruction::CLUI(rdindex, cnzimmediate) => {
                format!("C.LUI {:}, {:}", index_to_name(rdindex), cnzimmediate)
            }
            Instruction::CSRLI(rdindex, cnzuimmediate) => {
                format!("C.SRLI {:}, {:}", index_to_name(rdindex), cnzuimmediate)
            }
            Instruction::CSRAI(rdindex, cnzuimmediate) => {
                format!("C.SRAI {:}, {:}", index_to_name(rdindex), cnzuimmediate)
            }
            Instruction::CANDI(rdindex, cnzuimmediate) => {
                format!("C.ANDI {:}, {:}", index_to_name(rdindex), cnzuimmediate)
            }
            Instruction::CSUB(rdindex, rs2index) => format!(
                "C.SUB {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs2index)
            ),
            Instruction::CXOR(rdindex, rs2index) => format!(
                "C.XOR {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs2index)
            ),
            Instruction::COR(rdindex, rs2index) => format!(
                "C.OR {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs2index)
            ),
            Instruction::CAND(rdindex, rs2index) => format!(
                "C.AND {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs2index)
            ),
            Instruction::CJ(cjimmediate) => format!("C.J {cjimmediate:}"),
            Instruction::CBEQZ(rs1index, cimmediate) => {
                format!("C.BEQZ {:}, {:}", index_to_name(rs1index), cimmediate)
            }
            Instruction::CBNEZ(rs1index, cimmediate) => {
                format!("C.BNEZ {:}, {:}", index_to_name(rs1index), cimmediate)
            }
            Instruction::CSLLI(rdindex, cnzuimmediate) => {
                format!("C.SLLI {:}, {:}", index_to_name(rdindex), cnzuimmediate)
            }
            Instruction::CFLDSP(rdindex, cuimmediate) => {
                format!("C.FLDSP {:}, {:}", index_to_name(rdindex), cuimmediate)
            }
            Instruction::CLWSP(rdindex, cuimmediate) => {
                format!("C.LWSP {:}, {:}", index_to_name(rdindex), cuimmediate)
            }
            Instruction::CFLWSP(rdindex, cuimmediate) => {
                format!("C.FLWSP {:}, {:}", index_to_name(rdindex), cuimmediate)
            }
            Instruction::CJR(rs1index) => format!("C.JR {:}", index_to_name(rs1index)),
            Instruction::CMV(rdindex, rs2index) => format!(
                "C.MV {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs2index)
            ),
            Instruction::CEBREAK() => "C.EBREAK".to_string(),
            Instruction::CJALR(rs1index) => {
                format!("C.JALR {:}", index_to_name(rs1index))
            }
            Instruction::CADD(rdindex, rs2index) => format!(
                "C.ADD {:}, {:}",
                index_to_name(rdindex),
                index_to_name(rs2index)
            ),
            Instruction::CFSDSP(rs2index, cluimmediate) => {
                format!("C.FSDSP {:}, {:}", index_to_name(rs2index), cluimmediate)
            }
            Instruction::CSWSP(rs2index, cluimmediate) => {
                format!("C.SWSP {:}, {:}", index_to_name(rs2index), cluimmediate)
            }
            Instruction::CFSWSP(rs2index, cluimmediate) => {
                format!("C.FSWSP {:}, {:}", index_to_name(rs2index), cluimmediate)
            }
            Instruction::WFI() => "wfi".to_string(),
        }
    }
}
