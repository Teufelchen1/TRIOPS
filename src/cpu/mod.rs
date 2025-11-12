//! This file is scoped around the `CPU` struct.
//! If something can not be `impl CPU` it is considered out of scope.
use std::array;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

use elf::abi;
use elf::endian::AnyEndian;
use elf::ElfBytes;

use crate::events::{CpuJob, Event};
use crate::instructions::{decode, Instruction};
use crate::periph::MmapPeripheral;

use memory::Memory;

pub use register::{index_to_name, Register};

mod executer;
mod memory;
mod register;

const LOG_LENGTH: usize = 80;

pub struct CPU<T: MmapPeripheral> {
    pub register: Register,
    pub memory: Memory<T>,
    pub waits_for_interrupt: bool,
    instruction_log: [Option<(usize, Instruction)>; LOG_LENGTH],
}

impl<T: MmapPeripheral> CPU<T> {
    pub fn from_elf(file: &[u8], uart: T) -> Self {
        let mut cpu = Self {
            register: Register::default(),
            memory: Memory::default_hifive(uart),
            waits_for_interrupt: false,
            instruction_log: array::from_fn(|_| None),
        };
        cpu.register.csr.mie = 1;

        let elffile =
            ElfBytes::<AnyEndian>::minimal_parse(file).expect("Failed to parse provided ELF file");

        if let Some(segments) = elffile.segments() {
            for phdr in segments {
                if phdr.p_type == abi::PT_LOAD {
                    if let Ok(mut addr) = usize::try_from(phdr.p_paddr) {
                        if cpu.memory.is_rom(addr) {
                            for i in elffile.segment_data(&phdr).unwrap() {
                                cpu.memory.rom[addr - cpu.memory.rom_base] = *i;
                                addr += 1;
                            }
                        } else if cpu.memory.is_ram(addr) {
                            for i in elffile.segment_data(&phdr).unwrap() {
                                cpu.memory.ram[addr - cpu.memory.ram_base] = *i;
                                addr += 1;
                            }
                        }
                    } else {
                        panic!("Could not get PT_LOAD address in your ELF file.");
                    }
                }
            }
        } else {
            panic!("Could not find segments in your ELF file.");
        }

        cpu.register.pc =
            u32::try_from(elffile.ehdr.e_entry).expect("Failed to read start address e_entry");

        cpu
    }

    pub fn from_bin(file: &[u8], uart: T, entry_address: usize, base_address: usize) -> Self {
        let mut cpu = Self {
            register: Register::default(),
            memory: Memory::default_hifive(uart),
            waits_for_interrupt: false,
            instruction_log: array::from_fn(|_| None),
        };

        if cpu.memory.is_rom(base_address) {
            for (addr, i) in file.iter().enumerate() {
                cpu.memory.rom[base_address - cpu.memory.rom_base + addr] = *i;
            }
        } else if cpu.memory.is_ram(base_address) {
            for (addr, i) in file.iter().enumerate() {
                cpu.memory.ram[base_address - cpu.memory.ram_base + addr] = *i;
            }
        } else {
            panic!("The provided baseaddress was neither in ROM nor RAM.");
        }

        cpu.register.pc = entry_address as u32;

        cpu
    }

    pub fn instruction_at_addr(&self, addr: usize) -> anyhow::Result<Instruction> {
        decode(self.memory.read_word(addr)?)
    }

    pub fn current_instruction(&self) -> anyhow::Result<(usize, Instruction)> {
        if self.waits_for_interrupt {
            Ok(self
                .last_instruction()
                .expect("How did you start waiting for an interrupt without executing WFI first?")
                .clone())
        } else {
            let addr = self.register.pc as usize;
            let inst = self.instruction_at_addr(addr)?;
            Ok((addr, inst))
        }
    }

    #[allow(dead_code)]
    pub fn next_instruction(&self) -> anyhow::Result<(usize, Instruction)> {
        let (cur_addr, cur_inst) = self.current_instruction()?;
        let addr = {
            if cur_inst.is_compressed() {
                cur_addr + 2
            } else {
                cur_addr + 4
            }
        };
        let inst = self.instruction_at_addr(addr)?;
        Ok((addr, inst))
    }

    pub fn next_n_instructions(&self, n: usize) -> Vec<(usize, Result<Instruction, u32>)> {
        let mut instruction_list = Vec::new();
        let mut addr = self.register.pc as usize;
        for _ in 0..n {
            let cur_inst = self.instruction_at_addr(addr);
            if let Ok(inst) = cur_inst {
                let compressed = inst.is_compressed();
                instruction_list.push((addr, Ok(inst)));
                if compressed {
                    addr += 2;
                } else {
                    addr += 4;
                }
            } else {
                instruction_list.push((addr, Err(self.memory.read_word(addr).unwrap_or(0))));
                addr += 4;
            }
        }
        instruction_list
    }

    pub fn last_instruction(&self) -> Option<&(usize, Instruction)> {
        self.instruction_log.last().unwrap_or(&None).as_ref()
    }

    pub fn last_n_instructions(&self, n: usize) -> &[Option<(usize, Instruction)>] {
        if n > self.instruction_log.len() {
            &self.instruction_log
        } else {
            &self.instruction_log[self.instruction_log.len() - n..]
        }
    }

    fn exception(&mut self, reason: register::MCAUSE) {
        self.register
            .csr
            .mstatus_set_mpie(self.register.csr.mstatus_get_mie());
        self.register.csr.mstatus_set_mie(false);
        self.register.csr.mepc = self.register.pc;
        self.register.csr.mcause = reason as u32;
        self.register.pc = self.register.csr.mtvec;
        self.waits_for_interrupt = false;
    }

    /// Returns true if an interrupt has occured
    pub fn check_interrupts(&mut self) -> bool {
        // Interrupts are implicitly enabled when stalling the cpu due to WFI
        // Or directly enabled via MIE
        if self.waits_for_interrupt || self.register.csr.mstatus_get_mie() {
            if let Some(_reason) = self.memory.pending_interrupt() {
                self.exception(register::MCAUSE::MachineExternalInterrupt);
                return true;
            }
        }
        false
    }

    /// Returns true for all instructions except when executing ebreak.
    /// ebreak is used to signal the termination of the programm.
    pub fn step(&mut self) -> anyhow::Result<bool> {
        self.check_interrupts();
        // Stall when waiting for interrupts
        if self.waits_for_interrupt {
            Ok(true)
        } else {
            let (addr, inst) = self.current_instruction()?;
            self.exec(&inst, true, true)?;
            self.instruction_log.rotate_left(1);
            self.instruction_log[LOG_LENGTH - 1] = Some((addr, inst.clone()));
            Ok(!matches!(inst, Instruction::EBREAK()))
        }
    }
}

pub fn create_cpu_thread<T: MmapPeripheral + 'static>(
    cpu: &Arc<Mutex<CPU<T>>>,
    sender: Sender<Event>,
    receiver: Receiver<CpuJob>,
) -> JoinHandle<()> {
    let cpu2 = Arc::clone(cpu);
    spawn(move || cpu_executor(&cpu2, &sender, &receiver))
}

fn cpu_executor<T: MmapPeripheral>(
    cpu: &Arc<Mutex<CPU<T>>>,
    sender: &Sender<Event>,
    receiver: &Receiver<CpuJob>,
) {
    let mut autostep = false;
    loop {
        let cpu_waits_for_interrupt = { cpu.lock().unwrap().waits_for_interrupt };
        let job = if autostep && !cpu_waits_for_interrupt {
            match receiver.try_recv() {
                Ok(job) => job,
                Err(std::sync::mpsc::TryRecvError::Empty) => CpuJob::Step(307),
                Err(_e) => return,
            }
        } else {
            // If we wait for interrupt, we wait
            // If we don't wait for interrupt, we wait anyway for the next CpuJob
            match receiver.recv() {
                Ok(job) => job,
                Err(_e) => return,
            }
        };

        let steps = match job {
            CpuJob::Step(num) => {
                if num == 0 {
                    continue;
                }
                num
            }
            CpuJob::AutoStep => {
                autostep = true;
                continue;
            }
            CpuJob::Stop => {
                autostep = false;
                continue;
            }
            CpuJob::CheckInterrupts => {
                {
                    let mut cpu = cpu.lock().unwrap();
                    if cpu.check_interrupts() {
                        sender.send(Event::CpuStepComplete(true)).unwrap();
                    }
                }
                continue;
            }
        };

        let mut continue_exec = true;
        {
            let mut cpu = cpu.lock().unwrap();
            for _ in 0..steps {
                match cpu.step() {
                    Ok(con_exe) => {
                        if !con_exe {
                            continue_exec = false;
                            break;
                        }
                    }
                    Err(err) => {
                        sender.send(Event::CpuPanic(err)).unwrap();
                        return;
                    }
                }
            }
        }
        if continue_exec {
            let _ = sender.send(Event::CpuStepComplete(true));
        } else {
            let _ = sender.send(Event::CpuStepComplete(false));
            break;
        }
        std::thread::yield_now();
    }
}
