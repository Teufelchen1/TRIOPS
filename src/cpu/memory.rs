//! This file is scoped around the `Memory` struct.
//! If something can not be `impl Memory` it is considered out of scope.

use crate::events::IrqCause;

pub trait AddrBus {
    fn set_reservation(&mut self, addr: usize, value: u32);

    fn get_reservation(&mut self) -> Option<(usize, u32)>;

    fn del_reservation(&mut self);

    fn pending_interrupt(&self) -> Option<IrqCause>;

    fn is_ram(&self, addr: usize) -> bool;

    fn load_ram_at(&mut self, offset: usize, data: &[u8]);

    fn is_rom(&self, addr: usize) -> bool;

    fn load_rom_at(&mut self, offset: usize, data: &[u8]);

    fn load_at(&mut self, offset: usize, data: &[u8]);

    fn read_byte(&self, addr: usize) -> anyhow::Result<u32>;

    fn read_halfword(&self, index: usize) -> anyhow::Result<u32> {
        let halfword = (self.read_byte(index + 1)? << 8) + self.read_byte(index)?;
        Ok(halfword)
    }

    fn read_word(&self, index: usize) -> anyhow::Result<u32> {
        let word = (self.read_halfword(index + 2)? << 16) + self.read_halfword(index)?;
        Ok(word)
    }

    fn write_byte(&mut self, addr: usize, value: u32) -> anyhow::Result<()>;

    fn write_halfword(&mut self, index: usize, value: u32) -> anyhow::Result<()> {
        if 0x200_4000 <= index && index <= 0x0200_4007 {
            println!("Set half mtimecmp {index:X} for {value} seconds");
        }
        self.write_byte(index, value)?;
        self.write_byte(index + 1, value >> 8)?;
        Ok(())
    }

    fn write_word(&mut self, index: usize, value: u32) -> anyhow::Result<()> {
        if 0x200_4000 <= index && index <= 0x0200_4007 {
            println!("Set word mtimecmp {index:X} for {value} seconds");
        }
        self.write_halfword(index, value)?;
        self.write_halfword(index + 2, value >> 16)?;
        Ok(())
    }
}
