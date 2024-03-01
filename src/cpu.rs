use crate::instructions::Instruction;

use crate::decoder::decode;

use crate::executer::exec;

use crate::memory::Memory;

use crate::register::Register;

use elf::abi;
use elf::endian::AnyEndian;
use elf::ElfBytes;

pub struct CPU {
    pub register: Register,
    pub memory: Memory,
}

impl CPU {
    pub fn default(file: &[u8]) -> Self {
        let mut cpu = Self {
            register: Register::default(),
            memory: Memory::default_hifive(),
        };

        let elffile = ElfBytes::<AnyEndian>::minimal_parse(file).unwrap();

        for phdr in elffile.segments().unwrap() {
            if phdr.p_type == abi::PT_LOAD {
                let mut addr = usize::try_from(phdr.p_paddr).unwrap();
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
            }
        }

        cpu.register.pc = u32::try_from(elffile.ehdr.e_entry).unwrap();

        cpu
    }

    pub fn instruction_at_addr(&self, addr: usize) -> Result<Instruction, &'static str> {
        decode(self.memory.read_word(addr))
    }

    pub fn current_instruction(&self) -> (u32, Instruction) {
        let addr = self.register.pc;
        let inst = self.instruction_at_addr(addr as usize).unwrap();
        (addr, inst)
    }

    #[allow(dead_code)]
    pub fn next_instruction(&self) -> (u32, Instruction) {
        let (cur_addr, cur_inst) = self.current_instruction();
        let addr = {
            if cur_inst.is_compressed() {
                cur_addr + 2
            } else {
                cur_addr + 4
            }
        };
        let inst = self.instruction_at_addr(addr as usize).unwrap();
        (addr, inst)
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
                instruction_list.push((addr, Err(self.memory.read_word(addr))));
                addr += 4;
            }
        }
        instruction_list
    }

    pub fn step(&mut self) -> bool {
        let (_, inst) = self.current_instruction();
        exec(&mut self.register, &mut self.memory, &inst, true, true)
    }
}
