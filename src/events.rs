use anyhow::Error;
use crossterm::event::{KeyEvent, MouseEvent};

use crate::cpu::Register;
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
    CpuObserved(Addr, Instruction, Register),
    Interrupt(IrqCause),
}

pub fn cpu_observed_to_string(
    addr: Addr,
    inst: Instruction,
    regs: Register,
    addr2line: Option<&addr2line::Loader>,
) -> String {
    fn format_base(addr: usize, inst: &str, info: &str) -> String {
        format!("0x{addr:x} {inst:>5} {info}")
    }

    match inst {
        Instruction::JAL(_, immediate) | Instruction::CJAL(immediate) => {
            let destination = if immediate.is_negative() {
                addr.wrapping_sub(immediate.unsigned_abs() as usize)
            } else {
                addr.wrapping_add(immediate.unsigned_abs() as usize)
            };

            if let Some(location) = addr2line
                .and_then(|addr2line| addr2line.find_location(destination as u64).ok())
                .flatten()
            {
                let file = location.file.unwrap_or("???");
                let line = location.line.unwrap_or(0);
                format_base(
                    addr,
                    "C/JAL",
                    &format!("-> 0x{destination:x}:{file}:{line}"),
                )
            } else {
                format_base(addr, "C/JAL", &format!(" -> 0x{destination:x}"))
            }
        }
        Instruction::CJALR(rs) => {
            let destination = regs.read(rs);

            if let Some(location) = addr2line
                .and_then(|addr2line| addr2line.find_location(destination as u64).ok())
                .flatten()
            {
                let file = location.file.unwrap_or("???");
                let line = location.line.unwrap_or(0);
                format_base(
                    addr,
                    "CJALR",
                    &format!("-> 0x{destination:x}:{file}:{line}"),
                )
            } else {
                format_base(addr, "CJALR", &format!(" -> 0x{destination:x}"))
            }
        }
        Instruction::JALR(_, rs, immediate) => {
            let addr = regs.read(rs);
            let destination = if immediate.is_negative() {
                addr.wrapping_sub(immediate.unsigned_abs())
            } else {
                addr.wrapping_add(immediate.unsigned_abs())
            };

            if let Some(location) = addr2line
                .and_then(|addr2line| addr2line.find_location(destination as u64).ok())
                .flatten()
            {
                let file = location.file.unwrap_or("???");
                let line = location.line.unwrap_or(0);
                format_base(
                    addr as usize,
                    "JALR",
                    &format!("-> 0x{destination:x}:{file}:{line}"),
                )
            } else {
                format_base(addr as usize, "JALR", &format!(" -> 0x{destination:x}"))
            }
        }
        _ => "Not implemented".to_owned(),
    }
}
