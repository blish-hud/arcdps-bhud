use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
pub type ChannelType = Vec<u8>;

pub struct Device {
    sender: mpsc::Sender<ChannelType>,
}

impl Device {
    pub fn new(connection_string: &'static str) -> Device {
        let (tx, rx) = mpsc::channel();
        let device = Device { sender: tx };
        std::thread::spawn(move || loop {
            let func = rx.recv().unwrap();
            if let Ok(mut stream) = TcpStream::connect(connection_string) {
                let bytes = build_array(func.as_ref());
                let _ = stream.write(bytes.as_ref());
            }
        });

        device
    }

    pub fn send(&self, func: ChannelType) -> Result<(), mpsc::SendError<ChannelType>> {
        self.sender.send(func)
    }
}

fn build_array(bytes: &[u8]) -> Vec<u8> {
    [&bytes.len().to_le_bytes(), bytes].concat()
}
