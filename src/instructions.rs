use crate::decoder::{
    Bimmediate, CJimmediate, CLUimmediate, CNZUimmediate, CNZimmediate, CUimmediate, Cimmediate,
    Iimmediate, Jimmediate, RDindex, RS1index, RS2index, Simmediate, Uimmediate,
};
use crate::system::register_name;

#[derive(Debug)]
pub enum Instruction {
    /* RV32I */
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
    MRET(),
    /* Zicsr */
    CSRRW(RDindex, RS1index, Iimmediate),
    CSRRS(RDindex, RS1index, Iimmediate),
    CSRRC(RDindex, RS1index, Iimmediate),
    CSRRWI(RDindex, RS1index, Iimmediate),
    CSRRSI(RDindex, RS1index, Iimmediate),
    CSRRCI(RDindex, RS1index, Iimmediate),
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
    CADDI4SPN(RDindex, CNZUimmediate),
    CFLD(RDindex, RS1index, CUimmediate),
    CLQ(RDindex, RS1index, CUimmediate),
    CFW(RDindex, RS1index, CUimmediate),
    CFLW(RDindex, RS1index, CUimmediate),
    CLD(RDindex, RS1index, CUimmediate),
    CFSD(RDindex, RS1index, CUimmediate),
    CSQ(RDindex, RS1index, CUimmediate),
    CSW(RDindex, RS1index, CUimmediate),
    CFSW(RDindex, RS1index, CUimmediate),
    CSD(RDindex, RS1index, CUimmediate),
    /* Compressed Q2 */
    CNOP(RDindex, CNZimmediate),
    CADDI(RDindex, CNZimmediate),
    CJAL(CJimmediate),
    CADDIW(RDindex, Cimmediate),
    CLI(RDindex, Cimmediate),
    CADDI16SP(RDindex, CNZimmediate),
    CLUI(RDindex, CNZimmediate),
    CSRLI(RDindex, CNZUimmediate),
    CSRLI64(RDindex, CNZUimmediate),
    CSRAI(RDindex, CNZUimmediate),
    CSRAI64(RDindex, CNZUimmediate),
    CANDI(RDindex, CNZUimmediate),
    CSUB(RDindex, RS2index),
    CXOR(RDindex, RS2index),
    COR(RDindex, RS2index),
    CAND(RDindex, RS2index),
    CSUBW(RDindex, RS2index),
    CADDW(RDindex, RS2index),
    CJ(CJimmediate),
    CBEQZ(RS1index, Cimmediate),
    CBNEZ(RS1index, Cimmediate),
    /* Compressed Q3 */
    CSLLI(RDindex, CNZUimmediate),
    CSLLI64(RDindex, CNZUimmediate),
    CFLDSP(RDindex, CUimmediate),
    CLQSP(RDindex, CUimmediate),
    CLWSP(RDindex, CUimmediate),
    CFLWSP(RDindex, CUimmediate),
    CLDSP(RDindex, CUimmediate),
    CJR(RS1index),
    CMV(RDindex, RS2index),
    CEBREAK(),
    CJALR(RS1index),
    CADD(RDindex, RS2index),
    CFSDSP(RS2index, CLUimmediate),
    CSQSP(RS2index, CLUimmediate),
    CSWSP(RS2index, CLUimmediate),
    CFSWSP(RS2index, CLUimmediate),
    CSDSP(RS2index, CLUimmediate),
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
            _ => format!("{:?}", self),
        }
    }
}
