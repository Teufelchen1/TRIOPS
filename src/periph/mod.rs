//! Emulation of hardware peripherals is scoped for this file.
//! Currently, only memory mapped peripherals are available via `trait MmapPeripheral`.

use crate::events::IrqCause;

pub trait MmapPeripheral: Send {
    fn read_byte(&self, addr: usize) -> u8;

    fn write_byte(&mut self, offset: usize, value: u8);

    fn write_halfword(&mut self, index: usize, value: u32) {
        self.write_byte(index, value as u8);
        self.write_byte(index + 1, (value >> 8).try_into().unwrap());
    }

    fn write_word(&mut self, index: usize, value: u32) {
        self.write_halfword(index, value);
        self.write_halfword(index + 2, value >> 16);
    }
    fn pending_interrupt(&self) -> Option<IrqCause>;
}
