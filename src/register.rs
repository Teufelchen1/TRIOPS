use crate::decoder::Rindex;

pub fn index_to_name(register: Rindex) -> &'static str {
    match register {
        0x00 => "zero",
        0x01 => "ra",
        0x02 => "sp",
        0x03 => "gp",
        0x04 => "tp",
        0x05 => "t0",
        0x06 => "t1",
        0x07 => "t2",
        0x08 => "s0",
        0x09 => "s1",
        0x0A => "a0",
        0x0B => "a1",
        0x0C => "a2",
        0x0D => "a3",
        0x0E => "a4",
        0x0F => "a5",
        0x10 => "a6",
        0x11 => "a7",
        0x12 => "s2",
        0x13 => "s3",
        0x14 => "s4",
        0x15 => "s5",
        0x16 => "s6",
        0x17 => "s7",
        0x18 => "s8",
        0x19 => "s9",
        0x1A => "s10",
        0x1B => "s11",
        0x1C => "t3",
        0x1D => "t4",
        0x1E => "t5",
        0x1F => "t6",
        _ => panic!("Unkown register"),
    }
}

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

#[allow(clippy::manual_range_patterns)]
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
pub struct Register {
    regs: [u32; 32],
    pub csr: CSR,
    pub pc: u32,
}

impl Register {
    pub fn read(&self, index: Rindex) -> u32 {
        self.regs[index]
    }

    pub fn write(&mut self, index: Rindex, value: u32) {
        if index > 0 {
            self.regs[index] = value;
        }
    }

    pub fn to_string(&self, index: Rindex) -> String {
        format!(
            "{:}: 0x{:08X}({:})",
            index_to_name(index),
            self.regs[index],
            self.regs[index] as i32
        )
    }
}
