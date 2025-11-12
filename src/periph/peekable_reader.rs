use std::sync::mpsc::{self, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;

/// A wrapper for `mpsc::Receiver` that can be queried if there is data pending.
pub struct PeekableReader<T> {
    data_available: Arc<Mutex<Option<T>>>,
    reader: mpsc::Receiver<T>,
}

impl<T: Send + 'static> PeekableReader<T> {
    /// Creates a new `PeekableReader`
    /// Argument `f` function / closure is called repeatedly
    /// Should yield a new value everytime that is send to the "receiver"
    /// `f` is allowed to block infinitly
    pub fn new<F: Fn() -> T + Send + 'static>(read_data: F) -> Self {
        let (tx, rx): (mpsc::Sender<T>, mpsc::Receiver<T>) = mpsc::channel();
        let data_mux = Arc::new(Mutex::new(None));
        let data_mux_clone = data_mux.clone();
        thread::spawn(move || loop {
            let data = read_data();
            let mut data_available = data_mux_clone.lock().unwrap();
            if (*data_available).is_none() {
                *data_available = Some(data);
            } else if tx.send(data).is_err() {
                return;
            }
        });
        Self {
            data_available: data_mux,
            reader: rx,
        }
    }

    /// Returns true if new data is available
    /// Returns false if not.
    pub fn has_data(&self) -> bool {
        let mut data_available = self.data_available.lock().unwrap();
        if data_available.is_some() {
            return true;
        }
        match self.reader.try_recv() {
            Ok(val) => {
                *data_available = Some(val);
                true
            }
            Err(err) => match err {
                TryRecvError::Empty => false,
                TryRecvError::Disconnected => panic!(),
            },
        }
    }

    /// Equivalent to the `mpsc::Receiver::try_recv()`
    /// This method will never block the caller in order to wait for data to become available.
    pub fn try_recv(&self) -> Option<T> {
        if let Some(value) = self.data_available.lock().unwrap().take() {
            Some(value)
        } else {
            match self.reader.try_recv() {
                Ok(value) => Some(value),
                Err(err) => match err {
                    TryRecvError::Empty => None,
                    TryRecvError::Disconnected => panic!(),
                },
            }
        }
    }
}
