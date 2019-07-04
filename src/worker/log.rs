use super::device::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::Receiver;

static NAME: &'static str = "log";
static FILE_NAME: &'static str = "addons/bhud/errors.log";

pub fn new() {
    let action = |rx: Receiver<ChannelType>| {
        let mut file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME);
        loop {
            let content = rx.recv().unwrap();
            if file_res.is_err() {
                file_res = OpenOptions::new().append(true).create(true).open(FILE_NAME);
            }
            if let Ok(file) = &mut file_res {
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
            }
        }
    };

    gen_device(NAME, action);
}

pub fn send(content: ChannelType) {
    send_to_device(NAME, content);
}
