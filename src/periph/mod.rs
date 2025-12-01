//! Emulation of hardware peripherals is scoped for this file.
//! Currently, only memory mapped peripherals are available via `trait MmapPeripheral`.

pub type InterruptReason = u32;

pub trait MmapPeripheral: Send {
    fn read(&self, offset: usize) -> u8;
    fn write(&mut self, offset: usize, value: u8);
    fn pending_interrupt(&self) -> Option<InterruptReason>;
}
