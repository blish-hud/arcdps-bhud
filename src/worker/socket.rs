use super::device::*;
use super::log;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;

static NAME: &'static str = "tcp";
static CONNECTION_STRING: &'static str = "127.0.0.1:8214";

pub fn new() {
    let action = |rx: Receiver<ChannelType>| {
        let mut stream = TcpStream::connect(CONNECTION_STRING);
        loop {
            let data_to_send = rx.recv().unwrap();
            if stream.is_err() {
                stream = TcpStream::connect(CONNECTION_STRING);
            }
            if let Ok(tcp_stream) = &mut stream {
                let bytes = build_array(data_to_send.as_ref());
                let res = tcp_stream.write(bytes.as_ref());
                match res {
                    Ok(0) => {
                        log::send("0 bytes transfered\n".into());
                        stream = TcpStream::connect(CONNECTION_STRING);
                    }
                    Err(e) => {
                        log::send(format!("error writing to socket: {}\n", e).into());
                        stream = TcpStream::connect(CONNECTION_STRING);
                    }
                    _ => {}
                }
            }
        }
    };

    gen_device(NAME, action);
}

pub fn send(content: ChannelType) {
    send_to_device(NAME, content);
}

fn build_array(bytes: &[u8]) -> Vec<u8> {
    [&bytes.len().to_le_bytes(), bytes].concat()
}
