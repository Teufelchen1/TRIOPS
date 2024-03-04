use std::sync::mpsc;

pub trait MmapPeripheral {
	fn addr_base(&self) -> usize;
	fn addr_limit(&self) -> usize;
	fn read(&self, offset: usize) -> u8;
	fn write(&self, offset: usize, value: u8);
}

pub struct UartTty;
impl MmapPeripheral for UartTty {
    fn addr_base(&self) -> usize {
    	0x1000_0000
    }
	fn addr_limit(&self) -> usize {
		0x1000_0001
	}
    fn read(&self, _offset: usize) -> u8 {
    	0
    }
    fn write(&self, _offset: usize, value: u8) {
    	print!("{:}", value as char);
    }
}

pub struct UartBuffered {
	pub sink: mpsc::Sender<char>,
}
impl MmapPeripheral for UartBuffered {
    fn addr_base(&self) -> usize {
    	0x1000_0000
    }
	fn addr_limit(&self) -> usize {
		0x1000_0001
	}
    fn read(&self, _offset: usize) -> u8 {
    	0
    }
    fn write(&self, _offset: usize, value: u8) {
    	self.sink.send(value as char).unwrap();
    }
}

