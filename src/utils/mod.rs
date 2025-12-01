use std::sync::mpsc;

mod map_to_unixsocket;
mod peekable_channel;
mod peekable_reader;

pub use map_to_unixsocket::map_to_unixsocket;
pub use peekable_channel::PeekableChannel;
use peekable_reader::PeekableReader;

pub type IOChannel = (mpsc::Sender<u8>, mpsc::Receiver<u8>);
