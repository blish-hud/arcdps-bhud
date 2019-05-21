use named_pipe::PipeClient;
use std::sync::mpsc;
use stopwatch::Stopwatch;

const PIPE_PREFIX: &str = "\\\\.\\pipe\\";

pub struct Device {
    sender: mpsc::SyncSender<fn() -> String>,
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
                    let packet = func();
                    let elapsed = watch.elapsed();
                    if elapsed < one_ms {
                        std::thread::sleep(one_ms - elapsed);
                    }
                    let _ = client.write_async_owned(packet.into());
                }
            }
        });
        return device;
    }

    pub fn send(&self, func: fn() -> String) -> Result<(), mpsc::TrySendError<fn() -> String>> {
        self.sender.try_send(func)
    }
}
