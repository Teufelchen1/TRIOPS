use crate::decoder_q0;
use crate::decoder_q1;
use crate::decoder_q2;
use crate::decoder_q3;
use crate::instructions::Instruction;

pub type Rindex = usize;
pub type RDindex = Rindex;
pub type RS1index = Rindex;
pub type RS2index = Rindex;

pub type RS1value = u32;
pub type RS2value = u32;

pub type Iimmediate = u32;
pub type Simmediate = u32;
pub type Bimmediate = u32;
pub type Uimmediate = u32;
pub type Jimmediate = u32;

/* Compressed instructions immediates */
pub type CNZUimmediate = u32;
pub type CUimmediate = u32;
pub type CNZimmediate = u32;
pub type CJimmediate = u32;
pub type Cimmediate = u32;
pub type CLUimmediate = u32;

#[allow(clippy::too_many_lines)]
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
            panic!("Can't happen")
        }
    }
}
