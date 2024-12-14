//! Utilities that might be shared throug out TRIOPS
pub fn sign_extend(num: u32, bitnum: u32) -> u32 {
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
