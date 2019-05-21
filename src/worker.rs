use crate::pipeline::Arc;
use named_pipe::PipeClient;
use prost::Message;
use std::sync::mpsc;
use stopwatch::Stopwatch;

const PIPE_PREFIX: &str = "\\\\.\\pipe\\";

pub struct Device {
    sender: mpsc::SyncSender<fn() -> Arc>,
}

impl Device {
    pub fn new(name: &'static str) -> Device {
        let (tx, rx) = mpsc::sync_channel(1);
        let device = Device { sender: tx };
        std::thread::spawn(move || {
            let one_ms = std::time::Duration::from_millis(1);
            let mut watch = Stopwatch::new();
            loop {
                let func = rx.recv().unwrap();
                if let Ok(client) = PipeClient::connect(PIPE_PREFIX.to_string() + name) {
                    watch.restart();
                    let msg = func();
                    let mut buf = Vec::<u8>::new();
                    if let Ok(_) = msg.encode(&mut buf) {
                        let elapsed = watch.elapsed();
                        if elapsed < one_ms {
                            std::thread::sleep(one_ms - elapsed);
                        }
                        let _ = client.write_async_owned(buf);
                    }
                }
            }
        });
        device
    }

    pub fn send(&self, func: fn() -> Arc) -> Result<(), mpsc::TrySendError<fn() -> Arc>> {
        self.sender.try_send(func)
    }
}
