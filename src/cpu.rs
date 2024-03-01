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

    pub fn step(&mut self) -> bool {
        let inst = decode(self.memory.read_word(self.register.pc as usize)).unwrap();
        exec(&mut self.register, &mut self.memory, &inst, true, true)
    }
}
