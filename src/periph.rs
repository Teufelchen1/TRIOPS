use std::io;
use std::io::Read;
use std::sync::mpsc::{self, TryRecvError};

pub trait MmapPeripheral {
    fn read(&self, offset: usize) -> u8;
    fn write(&self, offset: usize, value: u8);
}

pub struct UartTty;
impl MmapPeripheral for UartTty {
    fn read(&self, _offset: usize) -> u8 {
        let mut buff: [u8; 1] = [0];
        if let Ok(count) = io::stdin().read(&mut buff) {
            if count == 0 {
                return 0;
            }
            return buff[0];
        }
        0
    }
    fn write(&self, _offset: usize, value: u8) {
        print!("{:}", value as char);
    }
}

pub struct UartBuffered {
    pub writer: mpsc::Sender<char>,
    pub reader: mpsc::Receiver<char>,
}
impl MmapPeripheral for UartBuffered {
    fn read(&self, _offset: usize) -> u8 {
        match self.reader.try_recv() {
            Ok(val) => return val as u8,
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => panic!(),
            },
        }
        0
    }
    fn write(&self, _offset: usize, value: u8) {
        self.writer.send(value as char).unwrap();
    }
}
