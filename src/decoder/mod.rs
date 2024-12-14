//! The decoder module is split into the four quadrants of possible risc-v instructions.
//! In each decoder, the instruction is classified into it's `OpCode` type.
//! See Chapter 34 in the The RISC-V Instruction Set Manual Volume I, Version 20240411.
//! Most instructions fall into the last quadrant number three, which contains the "normal"
//! 32 bit wide instructions.
mod decoder_q0;
mod decoder_q1;
mod decoder_q2;
mod decoder_q3;
use crate::instructions::Instruction;

pub type Rindex = usize;
pub type RDindex = Rindex;
pub type RS1index = Rindex;
pub type RS2index = Rindex;

pub type RS1value = u32;
pub type RS2value = u32;

pub type Immediate = i32;

pub fn bit_from_to(inst: u32, from: u32, to: u32) -> u32 {
    ((inst >> from) & 1) << to
}

pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    let encoding_quadrant = instruction & 0b11;
    match encoding_quadrant {
        0 => {
            /* compressed 16 bit wide */
            decoder_q0::decode(instruction & 0xFFFF)
        }
        1 => {
            /* compressed 16 bit wide */
            decoder_q1::decode(instruction & 0xFFFF)
        }
        2 => {
            /* compressed 16 bit wide */
            decoder_q2::decode(instruction & 0xFFFF)
        }
        3 => {
            /* regular 32 bit wide */
            decoder_q3::decode(instruction)
        }
        _ => {
            unreachable!()
        }
    }
}
