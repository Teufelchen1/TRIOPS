use std::fs;
use std::{io, thread};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
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

/// A backend that read/writes to a unix socket
pub struct BackendSocket {
    buffered_backend: BackendBuffered<u8>,
}

fn unixsocket_server(input: mpsc::Receiver<u8>, output: mpsc::Sender<u8>,) {
    let _ = fs::remove_file("./unixsocket");
    let listener = UnixListener::bind("./unixsocket").unwrap();
    if let Ok((mut socket, _addr)) = listener.accept() {
        println!("Accepted unixsocket listener");
        let mut socket2 = socket.try_clone().unwrap();
        thread::Builder::new()
            .name("Unixsocket Reader".to_owned())
            .spawn(move || {
                let mut buf: [u8; 1] = [0; 1];
                while let Ok(_num) = socket2.read_exact(&mut buf) {
                    output.send(buf[0]).unwrap();
                }
            })
            .unwrap();

        while let Ok(data) = input.recv() {
            socket.write(&[data]).unwrap();
        }
    }
    panic!("Unixsocket Server thread died");
}

impl BackendSocket {
    pub fn new(interrupts: mpsc::Sender<Event>) -> Self {
        let (from_unix_to_triops, triops_receive_from_unix) = mpsc::channel();
        let (from_triops_to_unix, unix_receive_from_triops) = mpsc::channel();

        thread::Builder::new()
            .name("Unixsocket Reader".to_owned())
            .spawn(move || unixsocket_server(unix_receive_from_triops, from_unix_to_triops))
            .unwrap();

        Self {
            buffered_backend: BackendBuffered::new(triops_receive_from_unix, from_triops_to_unix, interrupts)
        }
    }
}

impl PeripheralBackend for BackendSocket {
    fn has_data(&self) -> bool {
        self.buffered_backend.has_data()
    }
    fn read_cb(&self) -> Option<u8> {
       self.buffered_backend.read_cb()
    }

    // We can directly print in order to write to stdout
    fn write_cb(&self, value: u8) {
        self.buffered_backend.write_cb(value);
    }
}