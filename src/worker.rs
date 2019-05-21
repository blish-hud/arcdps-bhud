use crate::pipeline::arc;
use named_pipe::PipeClient;
use quick_protobuf::Writer;
use std::sync::mpsc;
use stopwatch::Stopwatch;

const PIPE_PREFIX: &str = "\\\\.\\pipe\\";

pub struct Device {
    sender: mpsc::SyncSender<fn() -> arc>,
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
                    let mut writer = Writer::new(client);
                    let msg = func();
                    let elapsed = watch.elapsed();
                    if elapsed < one_ms {
                        std::thread::sleep(one_ms - elapsed);
                    }
                    let _ = writer.write_message(&msg);
                }
            }
        });
        device
    }

    pub fn send(&self, func: fn() -> arc) -> Result<(), mpsc::TrySendError<fn() -> arc>> {
        self.sender.try_send(func)
    }
}
