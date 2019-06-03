use crate::pipeline::{arc::Msg, Arc, Mtype};
use named_pipe::PipeClient;
use prost::Message;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc as AtomicRc,
};
use stopwatch::Stopwatch;

const PIPE_PREFIX: &str = "\\\\.\\pipe\\";

pub struct Device {
    sender: mpsc::SyncSender<fn() -> Arc>,
}

impl Device {
    pub fn new(name: &'static str) -> Device {
        let (tx, rx) = mpsc::sync_channel(1);
        let device = Device { sender: tx };
        let one_ms = std::time::Duration::from_millis(1);
        let bhud_active = AtomicRc::new(AtomicBool::new(false));
        let bhud_share = bhud_active.clone();
        std::thread::spawn(move || loop {
            let mut found = false;
            for process_info in proclist::iterate_processes_info().filter_map(|r| r.ok()) {
                if &process_info.name == "Blish HUD.exe" {
                    found = true;
                    break;
                }
            }
            if found && !bhud_active.load(Ordering::Relaxed) {
                std::thread::sleep(one_ms * 30_000);
            }
            bhud_active.store(found, Ordering::Relaxed);
            std::thread::sleep(one_ms * 5000);
        });
        std::thread::spawn(move || {
            let mut was_active = false;
            let mut watch = Stopwatch::new();
            let mut send = |func: fn() -> Arc| {
                if let Ok(client) = PipeClient::connect(PIPE_PREFIX.to_string() + name) {
                    watch.restart();
                    let msg = func();
                    let mut buf = Vec::<u8>::new();
                    if msg.encode(&mut buf).is_ok() {
                        let elapsed = watch.elapsed();
                        if elapsed < one_ms {
                            std::thread::sleep(one_ms - elapsed);
                        }
                        let _ = client.write_async_owned(buf);
                    }
                }
            };
            loop {
                let func = rx.recv().unwrap();
                let active = bhud_share.load(Ordering::Relaxed);
                if active {
                    if !was_active {
                        send(|| Arc {
                            msgtype: Mtype::Greeting as i32,
                            msg: Some(Msg::Greeting(true)),
                        });
                        was_active = true;
                    }
                    send(func);
                } else {
                    was_active = false;
                }
            }
        });

        device
    }

    pub fn send(&self, func: fn() -> Arc) -> Result<(), mpsc::TrySendError<fn() -> Arc>> {
        self.sender.try_send(func)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let _ = self.send(|| Arc {
            msgtype: Mtype::Greeting as i32,
            msg: Some(Msg::Greeting(false)),
        });
    }
}
