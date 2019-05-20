use named_pipe::PipeClient;
use std::sync::mpsc;

const PIPE_PREFIX: &str = "\\\\.\\pipe\\";

pub struct Device {
    sender: mpsc::SyncSender<()>,
}

impl Device {
    pub fn new(name: &'static str) -> Device {
        let (tx, rx) = mpsc::sync_channel(1);
        let device = Device { sender: tx };
        std::thread::spawn(move || {
            let one_ms = std::time::Duration::from_millis(1);
            loop {
                rx.recv().unwrap();
                if let Ok(client) = PipeClient::connect(PIPE_PREFIX.to_string() + name) {
                    std::thread::sleep(one_ms);
                    let _ = client.write_async_owned("".into());
                }
            }
        });
        return device;
    }

    pub fn send(&self) -> Result<(), mpsc::TrySendError<()>> {
        self.sender.try_send(())
    }
}
