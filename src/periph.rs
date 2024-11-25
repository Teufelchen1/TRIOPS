use std::io;
use std::io::Read;
use std::sync::mpsc::{self, TryRecvError};

pub trait MmapPeripheral {
    fn read(&self, offset: usize) -> u8;
    fn write(&mut self, offset: usize, value: u8);
}

pub trait Backend {
    fn read_cb(&self) -> Option<u8>;
    fn write_cb(&self, value: u8);
}

#[allow(clippy::struct_excessive_bools)]
pub struct Uart<B> {
    tx_fifo_full: bool,
    rx_fifo_empty: bool,
    tx_enable: bool,
    rx_enable: bool,
    txcnt: u8,
    rxcnt: u8,
    txwm_ie: bool, // watermark interrupt enable
    rxwm_ie: bool,
    txwm_ip: bool, // watermark interrupt pending
    rxwm_ip: bool,
    backend: B,
}
impl<B> Uart<B> {
    pub fn default(backend: B) -> Self {
        Uart {
            tx_fifo_full: false,
            rx_fifo_empty: true,
            tx_enable: false,
            rx_enable: false,
            txcnt: 0,
            rxcnt: 0,
            txwm_ie: false, // watermark interrupt enable
            rxwm_ie: false,
            txwm_ip: false, // watermark interrupt pending
            rxwm_ip: false,
            backend,
        }
    }
}

impl<B: Backend> MmapPeripheral for Uart<B> {
    #[allow(clippy::match_same_arms)]
    fn read(&self, address_offset: usize) -> u8 {
        match address_offset {
            0x00..=0x02 => 0, // txdata Transmit data register
            0x03 => {
                // .31 txdata FIFO full bit
                if self.tx_fifo_full {
                    return 0b1000_0000;
                }
                0
            }
            0x04 => {
                // ...0x07 rxdata Receive data register
                if self.rx_enable {
                    if let Some(data) = self.backend.read_cb() {
                        return data;
                    }
                }
                0
            }
            0x05..=0x06 => 0,
            0x07 => {
                // .31 rxdata FIFO empty bit
                if self.rx_fifo_empty {
                    return 0b1000_0000;
                }
                0
            }
            0x08 => {
                // ...0x0B txctrl Transmit control register
                let mut ret = 0x00;
                if self.tx_enable {
                    ret |= 0x01; // tx enable
                }
                // ret |= 0x02; // num stopbits: Hardcoded always 0
                ret
            }
            0x09 => 0,
            0x0A => {
                // ...0x0B txctrl Transmit control register
                // Bit 16..17..18 are for txcnt / watermark
                self.txcnt & 0b111
            }
            0x0B => 0,
            0x0C => {
                // ...0x0F rxctrl Receive control register
                // First bit is rx en
                if self.rx_enable {
                    return 1;
                }
                0
            }
            0x0D => 0,
            0x0E => {
                // Bit 16..17..18 are for rxcnt
                self.rxcnt & 0b111
            }
            0x0F => 0,
            0x10 => {
                // ie UART interrupt enable
                let mut ret = 0x00;
                if self.txwm_ie {
                    ret |= 0x01; // txwm Transmit watermark interrupt enable
                }
                if self.rxwm_ie {
                    ret |= 0x02; // rxwm Receive watermark interrupt enable
                }
                ret
            }
            0x11..=0x13 => 0,
            0x14 => {
                // ip UART interrupt pending
                let mut ret = 0x00;
                if self.txwm_ip {
                    ret |= 0x01; // txwm Transmit watermark interrupt pending
                }
                if self.rxwm_ip {
                    ret |= 0x02; // rxwm Receive watermark interrupt pending
                }
                ret
            }
            0x15..=0x17 => 0,
            0x18 => todo!(), // div Baud rate divisor
            _ => panic!("UART read-access out of bounds: {address_offset:}"),
        }
    }

    #[allow(clippy::match_same_arms)]
    fn write(&mut self, address_offset: usize, value: u8) {
        match address_offset {
            0x00 => {
                // txdata Transmit data register
                if self.tx_enable {
                    self.backend.write_cb(value);
                }
            }
            0x01..=0x07 => (),
            0x08 => {
                // ...0x0B txctrl Transmit control register
                if (value & 0b1) != 0 {
                    self.tx_enable = true;
                }
                if (value & 0b10) != 0 {
                    // stopbits ignored
                }
            }
            0x09 => (),
            0x0A => {
                // ...0x0B txctrl Transmit control register
                // Bit 16..17..18 are for txcnt / watermark
                self.txcnt = value & 0b111;
            }
            0x0B => (),
            0x0C => {
                // ...0x0F rxctrl Receive control register
                // First bit is rx en
                if (value & 0b1) != 0 {
                    self.rx_enable = true;
                }
            }
            0x0D => (),
            0x0E => {
                // Bit 16..17..18 are for rxcnt
                self.rxcnt = value & 0b111;
            }
            0x0F => (),
            0x10 => {
                // ie UART interrupt enable
                self.txwm_ie = (value & 0b1) != 0;
                self.rxwm_ie = (value & 0b10) != 0;
            }
            0x11..=0x17 => (),
            0x18 => todo!(), // div Baud rate divisor
            _ => panic!("UART write-access out of bounds: {address_offset:}"),
        }
    }
}

pub struct UartTty {}

impl Backend for UartTty {
    fn read_cb(&self) -> Option<u8> {
        let mut buffer: [u8; 1] = [0];
        if let Ok(count) = io::stdin().read(&mut buffer) {
            if count != 0 {
                return Some(buffer[0]);
            }
        }
        None
    }
    fn write_cb(&self, value: u8) {
        print!("{:}", value as char);
    }
}

pub struct UartBuffered {
    pub writer: mpsc::Sender<char>,
    pub reader: mpsc::Receiver<char>,
}

impl Backend for UartBuffered {
    fn read_cb(&self) -> Option<u8> {
        match self.reader.try_recv() {
            Ok(val) => return Some(val as u8),
            Err(err) => match err {
                TryRecvError::Empty => {}
                TryRecvError::Disconnected => panic!(),
            },
        }
        None
    }
    fn write_cb(&self, value: u8) {
        self.writer.send(value as char).unwrap();
    }
}
