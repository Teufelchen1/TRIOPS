//! Emulation of hardware peripherals is scoped for this file.
//! Currently, only memory mapped peripherals are available via `trait MmapPeripheral`.
use std::sync::mpsc;

mod backend;
mod peekable_reader;
mod uart;

use backend::{BackendBuffered, BackendTty};
use uart::Uart;

use crate::events::Event;

type InterruptReason = u32;

pub trait MmapPeripheral: Send {
    fn read(&self, offset: usize) -> u8;
    fn write(&mut self, offset: usize, value: u8);
    fn pending_interrupt(&self) -> Option<InterruptReason>;
}

trait PeripheralBackend {
    fn has_data(&self) -> bool;
    fn read_cb(&self) -> Option<u8>;
    fn write_cb(&self, value: u8);
}

pub fn new_buffered_uart(
    input: mpsc::Receiver<u8>,
    output: mpsc::Sender<u8>,
    interrupt_queue: mpsc::Sender<Event>,
) -> impl MmapPeripheral {
    Uart::default(BackendBuffered::new(input, output, interrupt_queue))
}

pub fn new_stdio_uart(interrupt_queue: mpsc::Sender<Event>) -> impl MmapPeripheral {
    Uart::default(BackendTty::new(interrupt_queue))
}
