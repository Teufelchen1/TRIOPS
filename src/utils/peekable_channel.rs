use std::sync::mpsc;

use crate::events::Event;

use super::PeekableReader;

/// A backend that reads from a given `mpsc::Receiver` and writes to a given `mpsc::Sender`
/// Emits an event/interrupt whenever data arrives
pub struct PeekableChannel<T> {
    writer: mpsc::Sender<T>,
    peek_reader: PeekableReader<T>,
}

impl<T: Send + Default + 'static> PeekableChannel<T> {
    pub fn channel(
        interrupts: mpsc::Sender<Event>,
    ) -> ((mpsc::Sender<T>, mpsc::Receiver<T>), Self) {
        let (tx1, input): (mpsc::Sender<T>, mpsc::Receiver<T>) = mpsc::channel();
        let (output, rx2): (mpsc::Sender<T>, mpsc::Receiver<T>) = mpsc::channel();
        let reader = PeekableReader::new(move || {
            let data = input.recv().unwrap_or_else(|_| T::default());
            let _ = interrupts.send(Event::InterruptUart);
            data
        });
        let new_self = Self {
            writer: output,
            peek_reader: reader,
        };
        ((tx1, rx2), new_self)
    }

    pub fn has_data(&self) -> bool {
        self.peek_reader.has_data()
    }

    pub fn read_cb(&self) -> Option<T> {
        self.peek_reader.try_recv()
    }

    pub fn write_cb(&self, value: T) {
        self.writer.send(value).unwrap();
    }
}
