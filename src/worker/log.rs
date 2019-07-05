use super::device::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;

static NAME: &'static str = "log";
static FILE_PATH: &'static str = "addons/bhud";
static FILE_NAME: &'static str = "addons/bhud/errors.log";

pub fn new() {
    let action = |active: Arc<AtomicBool>, rx: Receiver<ChannelType>| {
        let _ = std::fs::create_dir_all(FILE_PATH);
        let mut file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME);
        let duration = Duration::new(1, 0);
        active.store(file_res.is_ok(), Release);
        loop {
            let content = if active.load(Acquire) {
                rx.recv().unwrap()
            } else {
                std::thread::sleep(duration);
                file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME);
                if file_res.is_ok() {
                    active.store(true, Release);
                }
                continue;
            };
            if file_res.is_err() {
                file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME);
            }
            if let Ok(file) = &mut file_res {
                active.store(true, Release);
                let res = file.write(content.as_ref());
                match res {
                    Ok(0) => {
                        file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME)
                    }
                    Err(_) => {
                        file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME)
                    }
                    _ => {}
                }
            } else {
                active.store(false, Release);
            }
        }
    };

    gen_device(NAME, action);
}

pub fn send(content: ChannelType) {
    send_to_device(NAME, content);
}
