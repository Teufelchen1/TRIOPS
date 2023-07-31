use crate::decoder::Rindex;

#[derive(Default)]
pub struct CSR {
    /* Machine Information Registers */
    pub mvendorid: u32,
    pub marchid: u32,
    pub mimpid: u32,
    pub mhartid: u32,
    pub mconfigptr: u32,
    /* Machine Trap Setup */
    pub mstatus: u32,
    pub misa: u32,
    pub medeleg: u32,
    pub mideleg: u32,
    pub mie: u32,
    pub mtvec: u32,
    pub mcounteren: u32,
    pub mstatush: u32,
    /* Machine Trap Handling */
    pub mscratch: u32,
    pub mepc: u32,
    pub mcause: u32,
    pub mtval: u32,
    pub mip: u32,
    pub mtinst: u32,
    pub mtval2: u32,
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
            0xF11 | 0xF12 | 0xF13 | 0xF14 | 0xF15 => {
                panic!("Attempt to write to read-only CSR!");
            }
            0x300 => {
                println!("Ingoring write of {value:X} into mstatus");
                self.mstatus = 0;
            }
            0x301 => {
                /* WARL / zero indicates misa is not implemented */
                self.misa = 0;
            }
            0x302 => {
                println!("Ingoring write of {value:X} into medeleg");
                self.medeleg = 0;
            }
            0x303 => {
                println!("Ingoring write of {value:X} into mideleg");
                self.mideleg = 0;
            }
            0x304 => {
                println!("Ingoring write of {value:X} into mie");
                self.mie = 0;
            }
            0x305 => {
                if value % 4 != 0 {
                    assert!(
                        value % 4 == 0,
                        "mtvec value not 4-byte aligned or mode other than Direct selected"
                    );
                }
                self.mtvec = value;
            }
            0x306 => {
                println!("Ingoring write of {value:X} into mcounteren");
                self.mcounteren = 0;
            }
            0x310 => {
                println!("Ingoring write of {value:X} into mstatush");
                self.mstatush = 0;
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
                println!("Ingoring write of {value:X} into mtval");
                self.mtval = 0;
            }
            0x344 => {
                println!("Ingoring write of {value:X} into mip");
                self.mip = 0;
            }
            0x34A => {
                println!("Ingoring write of {value:X} into mtinst");
                self.mtinst = 0;
            }
            0x34B => {
                println!("Ingoring write of {value:X} into mtval2");
                self.mtval2 = 0;
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

    pub fn to_string(&self, index: Rindex) -> String {
        match index {
            0x00 => format!("zero: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x01 => format!("ra: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x02 => format!("sp: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x03 => format!("gp: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x04 => format!("tp: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x05 => format!("t0: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x06 => format!("t1: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x07 => format!("t2: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x08 => format!("s0: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x09 => format!("s1: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x0A => format!("a0: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x0B => format!("a1: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x0C => format!("a2: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x0D => format!("a3: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x0E => format!("a4: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x0F => format!("a5: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x10 => format!("a6: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x11 => format!("a7: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x12 => format!("s2: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x13 => format!("s3: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x14 => format!("s4: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x15 => format!("s5: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x16 => format!("s6: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x17 => format!("s7: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x18 => format!("s8: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x19 => format!("s9: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x1A => format!("s10: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x1B => format!("s11: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x1C => format!("t3: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x1D => format!("t4: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x1E => format!("t5: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            0x1F => format!("t6: 0x{:08X}({:})", self.regs[index], self.regs[index] as i32),
            _ => panic!("Unkown registers"),
        }
    }
}

pub struct Memory {
    pub io_base: usize,
    pub io_limit: usize,
    pub ram_base: usize,
    pub ram_limit: usize,
    pub ram: Vec<u8>,
    pub rom_base: usize,
    pub rom_limit: usize,
    pub rom: Vec<u8>,
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
        panic!("Memory access outside memory map: 0x{addr:X}");
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
        panic!("Memory access outside memory map: 0x{addr:X}");
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
