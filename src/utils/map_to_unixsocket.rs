use std::io::Read;
use std::io::Write;
use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use super::IOChannel;

fn unixsocket_writer(input: &mpsc::Receiver<u8>, socket_receiver: &mpsc::Receiver<UnixStream>) {
    let mut maybe_socket: Option<UnixStream> = None;
    while let Ok(data) = input.recv() {
        loop {
            if maybe_socket.is_none() {
                maybe_socket = Some(socket_receiver.recv().unwrap());
            }
            if let Some(ref mut socket) = maybe_socket {
                if let Err(_e) = socket.write(&[data]) {
                    maybe_socket = None;
                } else {
                    break;
                }
            }
        }
    }
}

fn unixsocket_server(input: mpsc::Receiver<u8>, output: &mpsc::Sender<u8>, socket_path: PathBuf) {
    let (socket_sender, socket_receiver) = mpsc::channel();
    let _handle = thread::Builder::new()
        .name("Unixsocket Writer".to_owned())
        .spawn(move || unixsocket_writer(&input, &socket_receiver))
        .unwrap();

    let listener = UnixListener::bind(socket_path).unwrap();
    loop {
        if let Ok((mut socket, _addr)) = listener.accept() {
            let socket2 = socket.try_clone().unwrap();
            socket_sender.send(socket2).unwrap();

            let mut buf: [u8; 1] = [0; 1];
            while let Ok(_num) = socket.read_exact(&mut buf) {
                if output.send(buf[0]).is_err() {
                    println!("Aborting Unixsocket reader because internal channel got closed");
                    return;
                }
            }
            let _ = socket.shutdown(std::net::Shutdown::Both);
        }
    }
}

pub fn map_to_unixsocket(channel: IOChannel, socket_path: PathBuf) {
    let (output, input) = channel;
    let _handle = thread::Builder::new()
        .name("Unixsocket Reader".to_owned())
        .spawn(move || unixsocket_server(input, &output, socket_path))
        .unwrap();
}
