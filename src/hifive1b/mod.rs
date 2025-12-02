use std::sync::mpsc;

use crate::cpu::AddrBus;
use crate::events;
use crate::periph::MmapPeripheral;
use crate::utils::IOChannel;

use uart::Uart;

mod uart;

pub struct Hifive1b {
    pub uart0channel: Option<IOChannel>,
    pub uart1channel: Option<IOChannel>,
    pub memory: Option<Memory>,
}

impl Hifive1b {
    pub fn new(interrupts: mpsc::Sender<events::Event>) -> Self {
        let (uart0channel, uart0) = Uart::default(interrupts.clone());
        let (uart1channel, uart1) = Uart::default(interrupts);
        let memory = Memory::new(uart0, uart1);
        Self {
            uart0channel: Some(uart0channel),
            uart1channel: Some(uart1channel),
            memory: Some(memory),
        }
    }
}

pub struct Memory {
    clic_base: usize,
    clic_limit: usize,
    pub uart0_base: usize,
    pub uart0: Uart,
    pub uart0_limit: usize,
    pub uart1_base: usize,
    pub uart1: Uart,
    pub uart1_limit: usize,
    pub ram_base: usize,
    pub ram_limit: usize,
    pub ram: Vec<u8>,
    pub rom_base: usize,
    pub rom_limit: usize,
    pub rom: Vec<u8>,
    pub reservation: Option<(usize, u32)>,
}

impl Memory {
    pub fn new(uart0: Uart, uart1: Uart) -> Self {
        Self {
            clic_base: 0x200_0000,
            clic_limit: 0x200c000,
            uart0_base: 0x1001_3000,
            uart0,
            uart0_limit: 0x1001_301C,
            uart1_base: 0x1002_3000,
            uart1,
            uart1_limit: 0x1002_301C,
            rom_base: 0x2000_0000,
            rom_limit: 0x4000_0000,
            rom: vec![0; 0x2000_0000],
            ram_base: 0x8000_0000,
            ram_limit: 0x8000_8000,
            ram: vec![0; 0x8000],
            reservation: None,
        }
    }

    fn is_uart0(&self, addr: usize) -> bool {
        self.uart0_base <= addr && addr < self.uart0_limit
    }

    fn is_uart1(&self, addr: usize) -> bool {
        self.uart1_base <= addr && addr < self.uart1_limit
    }

    fn is_clic(&self, addr: usize) -> bool {
        self.clic_base <= addr && addr < self.clic_limit
    }
}

impl AddrBus for Memory {
    fn set_reservation(&mut self, addr: usize, value: u32) {
        self.reservation = Some((addr, value));
    }

    fn get_reservation(&mut self) -> Option<(usize, u32)> {
        self.reservation
    }

    fn del_reservation(&mut self) {
        self.reservation = None;
    }

    fn pending_interrupt(&self) -> Option<u32> {
        let i0 = self.uart0.pending_interrupt();
        if i0.is_some() {
            return i0;
        }
        let i1 = self.uart1.pending_interrupt();
        if i1.is_some() {
            return i1;
        }
        None
    }

    fn is_ram(&self, addr: usize) -> bool {
        self.ram_base <= addr && addr < self.ram_limit
    }

    fn load_ram_at(&mut self, offset: usize, data: &[u8]) {
        let ram = &mut self.ram[offset..];
        for (x, i) in data.iter().enumerate() {
            ram[x] = *i;
        }
    }

    fn is_rom(&self, addr: usize) -> bool {
        self.rom_base <= addr && addr < self.rom_limit
    }

    fn load_rom_at(&mut self, offset: usize, data: &[u8]) {
        let rom = &mut self.rom[offset..];
        for (x, i) in data.iter().enumerate() {
            rom[x] = *i;
        }
    }

    fn load_at(&mut self, addr: usize, data: &[u8]) {
        if self.is_ram(addr) {
            let offset = addr - self.ram_base;
            self.load_ram_at(offset, data);
            return;
        } else if self.is_rom(addr) {
            let offset = addr - self.rom_base;
            self.load_rom_at(offset, data);
            return;
        }
        panic!("Can't load at this address");
    }

    fn read_byte(&self, addr: usize) -> anyhow::Result<u32> {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            return Ok(u32::from(self.ram[index]));
        }
        if self.is_rom(addr) {
            let index = addr - self.rom_base;
            return Ok(u32::from(self.rom[index]));
        }
        if self.is_uart0(addr) {
            return Ok(u32::from(self.uart0.read(addr - self.uart0_base)));
        }
        if self.is_uart1(addr) {
            return Ok(u32::from(self.uart1.read(addr - self.uart1_base)));
        }
        if self.is_clic(addr) {
            return match addr {
                0x200_0000 => Ok(0), // msip for hart 0 MSIP Registers (1 bit wide)
                0x200_4000 => Ok(0), // mtimecmp for hart 0 MTIMECMP Registers
                0x200_bff8 => Ok(0), // mtime Timer register
                _ => Err(anyhow::anyhow!(
                    "Clic: attempted read outside memory map at address: 0x{addr:08X}"
                ))
            }
        }

        // FIXME: Temporal hack to get RIOT happy in-time for the 1.0 release
        #[allow(clippy::match_same_arms)]
        match addr {
            // PLIC
            0x0C20_0004 => {
                if self.uart0.pending_interrupt().is_some() {
                    Ok(0x03)
                } else if self.uart1.pending_interrupt().is_some() {
                    Ok(0x04)
                } else {
                    Ok(0x00)
                }
            }
            0x0C00_0000..=0x0FFF_FFFF => Ok(0x00),
            // RTT
            0x1000_0040..=0x1000_0080 => Ok(0x00),
            // PRCI
            0x1000_8000..=0x1000_800F => {
                // RIOT uses hfrosccfg, hfxosccfg, pllcfg, plloutdiv, procmoncfg
                Ok(0xFF)
            }
            // GPIO
            0x1001_2000..=0x1001_2FFF => Ok(0xFF),
            _ => Err(anyhow::anyhow!(
                "Memory: attempted read outside memory map at address: 0x{addr:08X}"
            )),
        }
    }

    fn write_byte(&mut self, addr: usize, value: u32) -> anyhow::Result<()> {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            self.ram[index] = (value & 0xFF) as u8;
            return Ok(());
        }
        if self.is_uart0(addr) {
            self.uart0
                .write(addr - self.uart0_base, (value & 0xFF) as u8);
            return Ok(());
        }
        if self.is_uart1(addr) {
            self.uart1
                .write(addr - self.uart1_base, (value & 0xFF) as u8);
            return Ok(());
        }

        // FIXME: Temporal hack to get RIOT happy in-time for the 1.0 release
        #[allow(clippy::match_same_arms)]
        match addr {
            // PLIC
            0x0C00_0000..=0x0FFF_FFFF => Ok(()),
            // RTT
            0x1000_0040..=0x1000_0080 => Ok(()),
            // PRCI
            0x1000_8000..=0x1000_800F => {
                // RIOT uses hfrosccfg, hfxosccfg, pllcfg, plloutdiv, procmoncfg
                Ok(())
            }
            // GPIO
            0x1001_2000..=0x1001_2FFF => Ok(()),
            // timer?
            0x0200_BFF8..=0x0200_BFFF => Ok(()),
            0x0200_4000..=0x0200_4007 => Ok(()),
            _ => Err(anyhow::anyhow!(
                "Memory: attempted write outside writable memory map at address: 0x{addr:08X}"
            )),
        }
    }
}
