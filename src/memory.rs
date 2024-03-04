use crate::periph::MmapPeripheral;

pub struct Memory {
    pub io_base: usize,
    pub io_limit: usize,
    pub ram_base: usize,
    pub ram_limit: usize,
    pub ram: Vec<u8>,
    pub rom_base: usize,
    pub rom_limit: usize,
    pub rom: Vec<u8>,
    pub periph: Vec<Box<dyn MmapPeripheral>>,
}

impl Memory {
    pub fn default_hifive() -> Self {
        Self {
            io_base: 0x0000_0000,
            io_limit: 0x2000_0000,
            rom_base: 0x2000_0000,
            rom_limit: 0x4000_0000,
            rom: vec![0; 0x2000_0000],
            ram_base: 0x8000_0000,
            ram_limit: 0x8000_4000,
            ram: vec![0; 0x4000],
            periph: Vec::<Box<dyn MmapPeripheral>>::new(),
        }
    }

    pub fn is_io(&self, addr: usize) -> bool {
        self.io_base <= addr && addr < self.io_limit
    }

    pub fn is_ram(&self, addr: usize) -> bool {
        self.ram_base <= addr && addr < self.ram_limit
    }

    pub fn is_rom(&self, addr: usize) -> bool {
        self.rom_base <= addr && addr < self.rom_limit
    }

    pub fn read_byte(&self, addr: usize) -> u32 {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            return u32::from(self.ram[index]);
        }
        if self.is_rom(addr) {
            let index = addr - self.rom_base;
            return u32::from(self.rom[index]);
        }
        if self.is_io(addr) {
            for periph in &self.periph {
                if periph.addr_base() <= addr && addr < periph.addr_limit() {
                    return u32::from(periph.read(addr - periph.addr_base()));
                }
            }
        }
        panic!("Memory read outside memory map: 0x{addr:X}");
    }
    pub fn read_halfword(&self, index: usize) -> u32 {
        (self.read_byte(index + 1) << 8) + self.read_byte(index)
    }
    pub fn read_word(&self, index: usize) -> u32 {
        (self.read_halfword(index + 2) << 16) + self.read_halfword(index)
    }
    pub fn write_byte(&mut self, addr: usize, value: u32) {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            self.ram[index] = (value & 0xFF) as u8;
            return;
        }
        if self.is_io(addr) {
            for periph in &self.periph {
                if periph.addr_base() <= addr && addr < periph.addr_limit() {
                    return periph.write(addr - periph.addr_base(), (value & 0xFF) as u8);
                }
            }
        }
        panic!("Memory write outside memory map: 0x{addr:X}");
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
