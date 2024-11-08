use crate::periph::MmapPeripheral;

pub struct Memory<'trait_periph> {
    pub uart_base: usize,
    pub uart: Option<&'trait_periph dyn MmapPeripheral>,
    pub uart_limit: usize,
    pub ram_base: usize,
    pub ram_limit: usize,
    pub ram: Vec<u8>,
    pub rom_base: usize,
    pub rom_limit: usize,
    pub rom: Vec<u8>,
}

impl Memory<'_> {
    pub fn default_hifive() -> Self {
        Self {
            uart_base: 0x1001_3000,
            uart: None,
            uart_limit: 0x1001_301C,
            rom_base: 0x2000_0000,
            rom_limit: 0x4000_0000,
            rom: vec![0; 0x2000_0000],
            ram_base: 0x8000_0000,
            ram_limit: 0x8000_4000,
            ram: vec![0; 0x4000],
        }
    }

    pub fn is_uart(&self, addr: usize) -> bool {
        self.uart_base <= addr && addr < self.uart_limit
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
        if self.is_uart(addr) {
            if let Some(uart) = self.uart {
                return u32::from(uart.read(addr - self.uart_base));
            }
            panic!("Memory read from non existing uart: 0x{addr:X}");
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
        if self.is_uart(addr) {
            if let Some(uart) = self.uart {
                return uart.write(addr - self.uart_base, (value & 0xFF) as u8);
            }
            panic!("Memory write to non existing uart: 0x{addr:X}");
        }
        panic!("Memory write outside writable memory map: 0x{addr:X}");
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
