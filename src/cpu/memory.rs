//! This file is scoped around the `Memory` struct.
//! If something can not be `impl Memory` it is considered out of scope.
use crate::periph::MmapPeripheral;

pub struct Memory<T> {
    pub uart_base: usize,
    pub uart: T,
    pub uart_limit: usize,
    pub ram_base: usize,
    pub ram_limit: usize,
    pub ram: Vec<u8>,
    pub rom_base: usize,
    pub rom_limit: usize,
    pub rom: Vec<u8>,
    pub reservation: Option<(usize, u32)>,
}

impl<T: MmapPeripheral> Memory<T> {
    pub fn default_hifive(uart: T) -> Self {
        Self {
            uart_base: 0x1001_3000,
            uart,
            uart_limit: 0x1001_301C,
            rom_base: 0x2000_0000,
            rom_limit: 0x4000_0000,
            rom: vec![0; 0x2000_0000],
            ram_base: 0x8000_0000,
            ram_limit: 0x8000_8000,
            ram: vec![0; 0x8000],
            reservation: None,
        }
    }

    pub fn pending_interrupt(&self) -> Option<u32> {
        self.uart.pending_interrupt()
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

    pub fn read_byte(&self, addr: usize) -> anyhow::Result<u32> {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            return Ok(u32::from(self.ram[index]));
        }
        if self.is_rom(addr) {
            let index = addr - self.rom_base;
            return Ok(u32::from(self.rom[index]));
        }
        if self.is_uart(addr) {
            return Ok(u32::from(self.uart.read(addr - self.uart_base)));
        }

        // FIXME: Temporal hack to get RIOT happy in-time for the 1.0 release
        #[allow(clippy::match_same_arms)]
        match addr {
            // PLIC
            0x0C20_0004 => {
                // Always ack UART0 interrupt for now
                Ok(0x03)
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
            // timer?
            0x0200BFF8..=0x0200BFFF => Ok(0),
            0x02004000..=0x02004003 => Ok(0),
            _ => Err(anyhow::anyhow!(
                "Memory: attempted read outside memory map at address: 0x{addr:08X}"
            )),
        }
    }
    pub fn read_halfword(&self, index: usize) -> anyhow::Result<u32> {
        let halfword = (self.read_byte(index + 1)? << 8) + self.read_byte(index)?;
        Ok(halfword)
    }
    pub fn read_word(&self, index: usize) -> anyhow::Result<u32> {
        let word = (self.read_halfword(index + 2)? << 16) + self.read_halfword(index)?;
        Ok(word)
    }
    pub fn write_byte(&mut self, addr: usize, value: u32) -> anyhow::Result<()> {
        if self.is_ram(addr) {
            let index = addr - self.ram_base;
            self.ram[index] = (value & 0xFF) as u8;
            return Ok(());
        }
        if self.is_uart(addr) {
            self.uart.write(addr - self.uart_base, (value & 0xFF) as u8);
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
            // Timer?
            0x0200BFF8..=0x0200BFFF => Ok(()),
            0x02004000..=0x02004007 => Ok(()),
            _ => Err(anyhow::anyhow!(
                "Memory: attempted write outside writable memory map at address: 0x{addr:08X}"
            )),
        }
    }
    pub fn write_halfword(&mut self, index: usize, value: u32) -> anyhow::Result<()> {
        self.write_byte(index, value)?;
        self.write_byte(index + 1, value >> 8)?;
        Ok(())
    }
    pub fn write_word(&mut self, index: usize, value: u32) -> anyhow::Result<()> {
        self.write_halfword(index, value)?;
        self.write_halfword(index + 2, value >> 16)?;
        Ok(())
    }
}
