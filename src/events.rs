use anyhow::Error;
use crossterm::event::{KeyEvent, MouseEvent};

pub enum CpuJob {
    Step(usize),
    AutoStep,
    Stop,
    CheckInterrupts,
}

pub enum Event {
    TerminalKey(KeyEvent),
    TerminalMouse(MouseEvent),
    TerminalResize,
    CpuStepComplete(bool),
    CpuPanic(Error),
    InterruptUart,
}
