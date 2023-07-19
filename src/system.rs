use crate::decoder::Rindex;

#[derive(Default)]
pub struct RegisterFile {
    regs: [u32; 32],
    pub pc: u32,
}

impl RegisterFile {
    pub fn read(&self, index: Rindex) -> u32 {
        return self.regs[index];
    }

    pub fn write(&mut self, index: Rindex, value: u32) {
        if index > 0 {
            self.regs[index] = value;
        }
    }
}

pub struct Memory {
    pub io_base: usize,
    pub io_len: usize,
    pub ram_base: usize,
    pub ram: [u8; 4096],
    pub rom_base: usize,
    pub rom: Vec<u8>,
}

impl Memory {
    pub fn default(rom: Vec<u8>) -> Self {
        Self {
            io_base: 0x6000_0000,
            io_len: 0x01,
            ram_base: 0x8000_0000,
            ram: [0; 4096],
            rom_base: 0x2000_0000,
            rom: rom,
        }
    }

    fn is_io(&self, addr: usize) -> bool {
        return self.io_base <= addr && addr < self.io_base + self.io_len;
    }

    fn is_ram(&self, addr: usize) -> bool {
        return self.ram_base <= addr && addr < self.ram_base + self.ram.len();
    }

    fn is_rom(&self, addr: usize) -> bool {
        return self.rom_base <= addr && addr < self.rom_base + self.rom.len();
    }

    pub fn read_byte(&self, addr: usize) -> u32 {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            return self.ram[index] as u32;
        }
        if self.is_rom(addr) {
            let index = addr - self.rom_base;
            return self.rom[index] as u32;
        }
        panic!("Memory access outside memory map: 0x{:X}", addr);
    }
    pub fn read_halfword(&self, index: usize) -> u32 {
        return (self.read_byte(index + 1) << 8) + self.read_byte(index);
    }
    pub fn read_word(&self, index: usize) -> u32 {
        return (self.read_halfword(index + 2) << 16) + self.read_halfword(index);
    }
    pub fn write_byte(&mut self, addr: usize, value: u32) {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            self.ram[index] = (value & 0xFF) as u8;
            return;
        }
        if self.is_io(addr) {
            print!("{:}", char::from_u32(value).unwrap());
            return;
        }
        panic!("Memory access outside memory map: 0x{:X}", addr);
    }
    pub fn write_halfword(&mut self, index: usize, value: u32) {
        self.write_byte(index, value);
        self.write_byte(index + 1, value >> 8);
    }
    pub fn write_word(&mut self, index: usize, value: u32) {
        self.write_halfword(index, value);
        self.write_halfword(index + 2, value >> 16);
    }
}
