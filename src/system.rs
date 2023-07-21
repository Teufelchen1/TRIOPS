use crate::decoder::Rindex;

#[derive(Default)]
pub struct CSR {
    /* Machine Information Registers */
    mvendorid: u32,
    marchid: u32,
    mimpid: u32,
    mhartid: u32,
    mconfigptr: u32,
    /* Machine Trap Setup */
    mstatus: u32,
    misa: u32,
    medeleg: u32,
    mideleg: u32,
    mie: u32,
    mtvec: u32,
    mcounteren: u32,
    mstatush: u32,
    /* Machine Trap Handling */
    mscratch: u32,
    mepc: u32,
    mcause: u32,
    mtval: u32,
    mip: u32,
    mtinst: u32,
    mtval2: u32,
}

impl CSR {
    pub fn read(&self, index: u32) -> u32 {
        match index {
            0xF11 => self.mvendorid,
            0xF12 => self.marchid,
            0xF13 => self.mimpid,
            0xF14 => self.mhartid,
            0xF15 => self.mconfigptr,
            0x300 => self.mstatus,
            0x301 => self.misa,
            0x302 => self.medeleg,
            0x303 => self.mideleg,
            0x304 => self.mie,
            0x305 => self.mtvec,
            0x306 => self.mcounteren,
            0x310 => self.mstatush,
            0x340 => self.mscratch,
            0x341 => self.mepc,
            0x342 => self.mcause,
            0x343 => self.mtval,
            0x344 => self.mip,
            0x34A => self.mtinst,
            0x34B => self.mtval2,
            _ => {
                todo!();
            }
        }
    }

    pub fn write(&mut self, index: u32, value: u32) {
        match index {
            0xF11 => {
                panic!("Attempt to write to read-only CSR!");
            }
            0xF12 => {
                panic!("Attempt to write to read-only CSR!");
            }
            0xF13 => {
                panic!("Attempt to write to read-only CSR!");
            }
            0xF14 => {
                panic!("Attempt to write to read-only CSR!");
            }
            0xF15 => {
                panic!("Attempt to write to read-only CSR!");
            }
            0x300 => {
                self.mstatus = value;
            }
            0x301 => {
                self.misa = value;
            }
            0x302 => {
                self.medeleg = value;
            }
            0x303 => {
                self.mideleg = value;
            }
            0x304 => {
                self.mie = value;
            }
            0x305 => {
                self.mtvec = value;
            }
            0x306 => {
                self.mcounteren = value;
            }
            0x310 => {
                self.mstatush = value;
            }
            0x340 => {
                self.mscratch = value;
            }
            0x341 => {
                self.mepc = value;
            }
            0x342 => {
                self.mcause = value;
            }
            0x343 => {
                self.mtval = value;
            }
            0x344 => {
                self.mip = value;
            }
            0x34A => {
                self.mtinst = value;
            }
            0x34B => {
                self.mtval2 = value;
            }
            _ => {
                todo!();
            }
        }
    }
}

#[derive(Default)]
pub struct RegisterFile {
    regs: [u32; 32],
    pub csr: CSR,
    pub pc: u32,
}

impl RegisterFile {
    pub fn read(&self, index: Rindex) -> u32 {
        self.regs[index]
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
            rom,
        }
    }

    fn is_io(&self, addr: usize) -> bool {
        self.io_base <= addr && addr < self.io_base + self.io_len
    }

    fn is_ram(&self, addr: usize) -> bool {
        self.ram_base <= addr && addr < self.ram_base + self.ram.len()
    }

    fn is_rom(&self, addr: usize) -> bool {
        self.rom_base <= addr && addr < self.rom_base + self.rom.len()
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
