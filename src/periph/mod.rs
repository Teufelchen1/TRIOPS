//! Emulation of hardware peripherals is scoped for this file.
//! Currently, only memory mapped peripherals are available via `trait MmapPeripheral`.

use crate::events::IrqCause;

pub trait MmapPeripheral: Send {
    fn read(&self, offset: usize) -> u8;
    fn write(&mut self, offset: usize, value: u8);
    fn pending_interrupt(&self) -> Option<IrqCause>;
}
