use anyhow::Error;
use crossterm::event::{KeyEvent, MouseEvent};

use crate::instructions::Instruction;

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

type Addr = usize;

pub enum Event {
    TerminalKey(KeyEvent),
    TerminalMouse(MouseEvent),
    TerminalResize,
    ExitApp,
    CpuStepComplete(bool),
    CpuPanic(Error),
    CpuObserved(Addr, Instruction),
    Interrupt(IrqCause),
}
