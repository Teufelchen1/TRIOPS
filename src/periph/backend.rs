use std::io;
use std::io::Read;
use std::sync::mpsc;

use crate::events::Event;
use crate::periph::peekable_reader::PeekableReader;
use crate::periph::PeripheralBackend;

/// A backend that reads from stdin and writes to stdout
pub struct BackendTty {
    peek_reader: PeekableReader<u8>,
}

impl BackendTty {
    pub fn new(interrupts: mpsc::Sender<Event>) -> Self {
        // setup a PeekableReader that fetches data from stdin
        let reader = PeekableReader::new(move || {
            let mut buffer: [u8; 1] = [0];
            io::stdin().read_exact(&mut buffer).unwrap();
            let _ = interrupts.send(Event::InterruptUart);
            buffer[0]
        });
        BackendTty {
            peek_reader: reader,
        }
    }
}

impl PeripheralBackend for BackendTty {
    fn has_data(&self) -> bool {
        self.peek_reader.has_data()
    }
    fn read_cb(&self) -> Option<u8> {
        if let Some(val) = self.peek_reader.try_recv() {
            return Some(val);
        }
        None
    }

    // We can directly print in order to write to stdout
    fn write_cb(&self, value: u8) {
        print!("{:}", value as char);
    }
}

/// A backend that reads from a given `mpsc::Receiver` and writes to a given `mpsc::Sender`
pub struct BackendBuffered<T> {
    writer: mpsc::Sender<T>,
    peek_reader: PeekableReader<T>,
}

impl<T: Send + 'static> BackendBuffered<T> {
    pub fn new(
        input: mpsc::Receiver<T>,
        output: mpsc::Sender<T>,
        interrupts: mpsc::Sender<Event>,
    ) -> Self {
        let reader = PeekableReader::new(move || {
            let data = input.recv().unwrap();
            let _ = interrupts.send(Event::InterruptUart);
            data
        });
        BackendBuffered {
            writer: output,
            peek_reader: reader,
        }
    }
}

impl<T: Send + 'static + From<u8>> PeripheralBackend for BackendBuffered<T>
where
    u8: From<T>,
{
    fn has_data(&self) -> bool {
        self.peek_reader.has_data()
    }
    fn read_cb(&self) -> Option<u8> {
        if let Some(val) = self.peek_reader.try_recv() {
            return Some(u8::from(val));
        }
        None
    }
    fn write_cb(&self, value: u8) {
        self.writer.send(value.into()).unwrap();
    }
}
