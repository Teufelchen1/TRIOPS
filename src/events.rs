use anyhow::Error;
use crossterm::event::{KeyEvent, MouseEvent};

pub enum CpuJob {
    Step(usize),
    AutoStep,
    Stop,
    CheckInterrupts,
}

#[derive(Clone)]
pub enum IrqCause {
    Uart,
    Timer,
}

pub enum Event {
    TerminalKey(KeyEvent),
    TerminalMouse(MouseEvent),
    TerminalResize,
    ExitApp,
    CpuStepComplete(bool),
    CpuPanic(Error),
    Interrupt(IrqCause),
}
